#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::str::from_utf8;
use world_core::content::parsing::markdown_to_elements;

static CRDT_BYTES: &[u8] = include_bytes!("data/xi-editor-crdt.md");

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("markdown_to_json", |b| {
        let input = from_utf8(CRDT_BYTES).unwrap();
        b.iter(|| {
            markdown_to_elements(input.to_string());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
