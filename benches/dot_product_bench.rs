use criterion::{criterion_group, criterion_main, Criterion};
fn mul(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    x.iter().zip(y.iter()).map(|(&a, &b)| a * b).sum()
}




fn bench_prime_nos(c: &mut Criterion) {
    let mut input1 = Vec::new();
    let mut input2 = Vec::new();
    for i in 1..2000{
        input1.push(i as f64);
        input2.push(i as f64);
    }
    c.bench_function("Prime No Benchmark", |b| b.iter(|| mul(&input1, &input2) ));
}

// Create a benchmark group
criterion_group!(benches, bench_prime_nos);

// Run the benchmarks
criterion_main!(benches);