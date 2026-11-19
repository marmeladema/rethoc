//! Rebar runner for counting matching lines.

use std::io::{self, Write};

use anyhow::{Context, ensure};
use bstr::ByteSlice;
use regex_syntax::ParserBuilder;

use rethoc_engine::{MatcherMemory, RegexBuilder};

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout().lock();
    if std::env::args().nth(1).as_deref() == Some("--version") {
        writeln!(stdout, "{}", env!("CARGO_PKG_VERSION"))?;
        return Ok(());
    }

    let benchmark = klv::Benchmark::read(io::stdin().lock()).context("read rebar input")?;
    ensure!(benchmark.model == "grep", "expected the grep model");
    let hir = ParserBuilder::new()
        .utf8(false)
        .unicode(benchmark.regex.unicode)
        .case_insensitive(benchmark.regex.case_insensitive)
        .build()
        .parse(benchmark.regex.one()?)?;
    let regex = RegexBuilder::default().build(&hir)?;
    let mut memory = MatcherMemory::default();
    let samples = timer::run(&benchmark, || {
        let mut count = 0;
        for line in benchmark.haystack.lines() {
            let mut matcher = memory.matcher(&regex);
            matcher.chunk(line);
            count += usize::from(matcher.finish());
        }
        Ok(count)
    })?;
    for sample in samples {
        writeln!(stdout, "{},{}", sample.duration.as_nanos(), sample.count)?;
    }
    Ok(())
}
