//! Run `rethoc` child processes under a memory and time cap.
//!
//! The memory cap is `ulimit -v` applied by `sh` before it `exec`s the
//! child, so it needs a POSIX shell and a kernel that honours
//! `RLIMIT_AS` (Linux).  The time cap is enforced here: the child is
//! killed at the deadline.  A pattern that blows up therefore costs one
//! child, not the whole run.

use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use bpaf::Bpaf;

use super::{EXIT_ABORT, EXIT_MISMATCH, EXIT_OK, EXIT_REJECTED};

/// Child processes:
#[derive(Bpaf, Clone, Debug)]
pub(crate) struct CapArgs {
    /// Parallel child processes (default: available CPUs)
    #[bpaf(short, long, argument("N"))]
    pub(crate) jobs: Option<usize>,
    /// Virtual memory cap per child, in KiB (`ulimit -v`; 0 disables)
    #[bpaf(long, argument("KIB"))]
    pub(crate) mem_limit: Option<u64>,
    /// Wall-clock cap per child, in seconds (default: 20 for sweep; for
    /// fuzz, the batch size times --abort-after, plus 30)
    #[bpaf(long, argument("SECS"))]
    pub(crate) timeout: Option<u64>,
}

impl CapArgs {
    /// Number of parallel children: `--jobs`, or one per CPU.
    pub(crate) fn jobs(&self) -> usize {
        self.jobs
            .unwrap_or_else(|| thread::available_parallelism().map_or(1, |n| n.get()))
            .max(1)
    }

    /// The caps for one child, using the given defaults for flags that
    /// were not passed.
    pub(crate) fn caps(&self, default_mem_kib: u64, default_timeout_secs: u64) -> Caps {
        Caps {
            mem_kib: Some(self.mem_limit.unwrap_or(default_mem_kib)).filter(|&m| m > 0),
            timeout: Duration::from_secs(self.timeout.unwrap_or(default_timeout_secs)),
        }
    }
}

/// Memory and time cap of one child.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Caps {
    /// `ulimit -v` value in KiB; `None` for no memory cap.
    pub(crate) mem_kib: Option<u64>,
    /// Wall-clock limit; the child is killed when it is reached.
    pub(crate) timeout: Duration,
}

/// How a child ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Verdict {
    /// Exit code 0.
    Ok,
    /// Exit code 1.
    Mismatch,
    /// Exit code 4.
    Rejected,
    /// The child's own watchdog gave up (exit code 3).
    Watchdog,
    /// Killed at the deadline.
    Timeout,
    /// An allocation failed under the memory cap.
    OutOfMemory,
    /// Exit code 101.
    Panic,
    /// Any other exit code or signal.
    Crash,
}

impl Verdict {
    /// Upper-case name used in reports.
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Mismatch => "MISMATCH",
            Self::Rejected => "REJECTED",
            Self::Watchdog => "ABORT",
            Self::Timeout => "TIMEOUT",
            Self::OutOfMemory => "OOM",
            Self::Panic => "PANIC",
            Self::Crash => "CRASH",
        }
    }
}

/// Result of one child.
pub(crate) struct Run {
    /// How the child ended.
    pub(crate) verdict: Verdict,
    /// Everything the child printed to stdout.
    pub(crate) stdout: String,
    /// Everything the child printed to stderr.
    pub(crate) stderr: String,
    /// Wall-clock time from spawn to exit.
    pub(crate) elapsed: Duration,
}

impl Run {
    /// A one-line reason: the first interesting line of output.
    pub(crate) fn detail(&self) -> String {
        let keys = [
            "MISMATCH",
            "ABORT",
            "memory allocation",
            "Cannot allocate memory",
            "panicked",
            "error",
        ];
        self.stdout
            .lines()
            .chain(self.stderr.lines())
            .find(|l| keys.iter().any(|k| l.contains(k)))
            .or_else(|| self.stderr.lines().last())
            .unwrap_or("")
            .trim()
            .to_string()
    }
}

/// Run `rethoc <args>` under `caps`.
///
/// Never panics: failing to start or wait for the child gives a
/// [`Verdict::Crash`] whose `stderr` holds the error.
pub(crate) fn run_capped(args: &[String], caps: Caps) -> Run {
    let start = Instant::now();
    try_run_capped(args, caps, start).unwrap_or_else(|e| Run {
        verdict: Verdict::Crash,
        stdout: String::new(),
        stderr: format!("error: {e}"),
        elapsed: start.elapsed(),
    })
}

fn try_run_capped(args: &[String], caps: Caps, start: Instant) -> Result<Run, String> {
    let exe = std::env::current_exe().map_err(|e| format!("cannot find the rethoc binary: {e}"))?;
    let mut cmd = match caps.mem_kib {
        Some(kib) => {
            let mut c = Command::new("sh");
            c.arg("-c")
                .arg(r#"ulimit -v "$1" || exit 125; shift; exec "$@""#)
                .arg("sh")
                .arg(kib.to_string())
                .arg(exe);
            c
        }
        None => Command::new(exe),
    };
    cmd.args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("cannot start a child process: {e}"))?;
    let stdout = drain(child.stdout.take());
    let stderr = drain(child.stderr.take());
    let mut timed_out = false;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {}
            Err(e) => {
                let _ = child.kill();
                return Err(format!("cannot wait for a child process: {e}"));
            }
        }
        if start.elapsed() >= caps.timeout {
            timed_out = true;
            let _ = child.kill();
            break child
                .wait()
                .map_err(|e| format!("cannot wait for a killed child process: {e}"))?;
        }
        thread::sleep(Duration::from_millis(2));
    };
    let elapsed = start.elapsed();
    let stdout = stdout.join().unwrap_or_default();
    let stderr = stderr.join().unwrap_or_default();

    let verdict = if timed_out {
        Verdict::Timeout
    } else if caps.mem_kib.is_some() && is_out_of_memory(&stderr) {
        Verdict::OutOfMemory
    } else {
        match status.code() {
            Some(EXIT_OK) => Verdict::Ok,
            Some(EXIT_MISMATCH) => Verdict::Mismatch,
            Some(EXIT_REJECTED) => Verdict::Rejected,
            Some(EXIT_ABORT) => Verdict::Watchdog,
            Some(101) => Verdict::Panic,
            _ => Verdict::Crash,
        }
    };
    Ok(Run {
        verdict,
        stdout,
        stderr,
        elapsed,
    })
}

/// Whether `stderr` shows an allocation failing.  Same patterns as
/// `scripts/capped`: Rust's allocation-failure abort, and ENOMEM
/// surfacing as an error (failed thread spawn or `mmap`).
fn is_out_of_memory(stderr: &str) -> bool {
    [
        "memory allocation of",
        "Cannot allocate memory",
        "out of memory",
        "Out of memory",
    ]
    .iter()
    .any(|p| stderr.contains(p))
}

/// Read a child's stdout or stderr to the end on a separate thread.
fn drain(pipe: Option<impl Read + Send + 'static>) -> thread::JoinHandle<String> {
    thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut pipe) = pipe {
            let _ = pipe.read_to_end(&mut buf);
        }
        String::from_utf8_lossy(&buf).into_owned()
    })
}

/// Run `work` on every task with `jobs` threads; `done` sees each result
/// on the calling thread, in completion order.
pub(crate) fn parallel<T: Sync, R: Send>(
    tasks: &[T],
    jobs: usize,
    work: impl Fn(&T) -> R + Sync,
    mut done: impl FnMut(&T, R),
) {
    let next = AtomicUsize::new(0);
    let (tx, rx) = mpsc::channel();
    thread::scope(|s| {
        for _ in 0..jobs.min(tasks.len()) {
            let tx = tx.clone();
            let (next, work) = (&next, &work);
            s.spawn(move || {
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(task) = tasks.get(i) else { break };
                    if tx.send((i, work(task))).is_err() {
                        break;
                    }
                }
            });
        }
        drop(tx);
        for (i, result) in rx {
            done(&tasks[i], result);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::is_out_of_memory;

    #[test]
    fn out_of_memory_patterns() {
        assert!(is_out_of_memory(
            "memory allocation of 268435456 bytes failed\n"
        ));
        assert!(is_out_of_memory(
            "failed to spawn thread: Os { code: 12, kind: OutOfMemory, message: \"Cannot allocate memory\" }"
        ));
        assert!(!is_out_of_memory(
            "thread 'main' panicked at src/lib.rs:1:1"
        ));
    }
}
