#[macro_use]
extern crate serde_derive;

use criterion::{criterion_group, criterion_main, Benchmark, Criterion};

use desse::{Desse, DesseSized};

#[derive(Desse, DesseSized)]
struct MyDesseStruct {
    a: u8,
    b: u16,
}

use bincode::serialize;

#[derive(Serialize, Deserialize)]
struct MySerdeStruct {
    a: u8,
    b: u16,
}

#[allow(unused_must_use)]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "struct::serialize",
        Benchmark::new("desse::serialize", |b| {
            b.iter(|| {
                let my_struct: MyDesseStruct = MyDesseStruct { a: 253, b: 64016 };
                Desse::serialize(&my_struct);
            })
        })
        .with_function("bincode::serialize", |b| {
            b.iter(|| {
                let my_struct: MySerdeStruct = MySerdeStruct { a: 253, b: 64016 };
                serialize(&my_struct);
            })
        }),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
