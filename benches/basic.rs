use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::{thread_rng, Rng};
use stash::Stash;
use stash::UniqueStash;
use store::Store;

fn fresh_inserts(c: &mut Criterion) {
    let size = 10_000;
    let mut g = c.benchmark_group("Fresh Inserts");
    let s: Store<usize, usize> = Store::new();
    g.bench_function("Store", |b| {
        b.iter_batched_ref(
            || s.clone(),
            |input| {
                for a in 0..size {
                    input.insert(a);
                }
            },
            BatchSize::SmallInput,
        );
    });
    let s: Stash<usize, usize> = Stash::new();
    g.bench_function("Stash", |b| {
        b.iter_batched_ref(
            || s.clone(),
            |input| {
                for a in 0..size {
                    input.put(a);
                }
            },
            BatchSize::SmallInput,
        );
    });
    let s: UniqueStash<usize> = UniqueStash::new();
    g.bench_function("UniqueStash", |b| {
        b.iter_batched_ref(
            || s.clone(),
            |input| {
                for a in 0..size {
                    input.put(a);
                }
            },
            BatchSize::SmallInput,
        );
    });
}

fn remove(c: &mut Criterion) {
    let size = 10_000;
    let mut rng = thread_rng();
    let mut s1: Store<usize, usize> = Store::new();
    let mut s2: Stash<usize, usize> = Stash::new();
    let mut s3: UniqueStash<usize> = UniqueStash::new();
    let mut s3k = Vec::new();
    for a in 0..size {
        s1.insert(a);
        s2.put(a);
        s3k.push(s3.put(a));
    }

    let mut g = c.benchmark_group("Remove");
    g.bench_function("Store", |b| {
        b.iter_batched_ref(
            || s1.clone(),
            |input| {
                for _ in 0..size {
                    input.remove(rng.gen_range(0, size));
                }
            },
            BatchSize::SmallInput,
        );
    });
    g.bench_function("Stash", |b| {
        b.iter_batched_ref(
            || s2.clone(),
            |input| {
                for _ in 0..size {
                    input.take(rng.gen_range(0, size));
                }
            },
            BatchSize::SmallInput,
        );
    });
    g.bench_function("UniqueStash", |b| {
        b.iter_batched_ref(
            || s3.clone(),
            |input| {
                for _ in 0..size {
                    input.take(s3k[rng.gen_range(0, size)]);
                }
            },
            BatchSize::SmallInput,
        );
    });
}

fn get(c: &mut Criterion) {
    let size = 10_000;
    let mut rng = thread_rng();
    let mut s1: Store<usize, usize> = Store::new();
    let mut s2: Stash<usize, usize> = Stash::new();
    let mut s3: UniqueStash<usize> = UniqueStash::new();
    let mut s3k = Vec::new();
    for a in 0..size {
        s1.insert(a);
        s2.put(a);
        s3k.push(s3.put(a));
    }
    let mut g = c.benchmark_group("Get");
    g.bench_function("Store", |b| {
        b.iter_batched_ref(
            || s1.clone(),
            |input| {
                for _ in 0..size {
                    black_box(input.get(rng.gen_range(0, size)));
                }
            },
            BatchSize::SmallInput,
        );
    });
    g.bench_function("Stash", |b| {
        b.iter_batched_ref(
            || s2.clone(),
            |input| {
                for _ in 0..size {
                    black_box(input.get(rng.gen_range(0, size)));
                }
            },
            BatchSize::SmallInput,
        );
    });
    g.bench_function("UniqueStash", |b| {
        b.iter_batched_ref(
            || s3.clone(),
            |input| {
                for _ in 0..size {
                    black_box(input.get(s3k[rng.gen_range(0, size)]));
                }
            },
            BatchSize::SmallInput,
        );
    });
}

fn iter(c: &mut Criterion) {
    let size = 10_000;
    let mut rng = thread_rng();
    let mut s1: Store<usize, usize> = Store::new();
    let mut s2: Stash<usize, usize> = Stash::new();
    let mut s3: UniqueStash<usize> = UniqueStash::new();
    let mut s3k = Vec::new();
    for a in 0..size {
        s1.insert(a);
        s2.put(a);
        s3k.push(s3.put(a));
    }

    let mut g = c.benchmark_group("Iterate");
    g.bench_function("Store", |b| {
        b.iter_batched_ref(
            || s1.clone(),
            |input| {
                for a in input.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
    g.bench_function("Stash", |b| {
        b.iter_batched_ref(
            || s2.clone(),
            |input| {
                for a in input.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
    g.bench_function("UniqueStash", |b| {
        b.iter_batched_ref(
            || s3.clone(),
            |input| {
                for a in input.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
    g.finish();

    for _ in 0..size / 2 {
        let k = rng.gen_range(0, size);
        s1.remove(k);
        s2.take(k);
        s3.take(s3k[k]);
    }

    let mut g = c.benchmark_group("Iterate half-full");
    g.bench_function("Store", |b| {
        b.iter_batched_ref(
            || s1.clone(),
            |input| {
                for a in input.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
    g.bench_function("Stash", |b| {
        b.iter_batched_ref(
            || s2.clone(),
            |input| {
                for a in input.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
    g.bench_function("UniqueStash", |b| {
        b.iter_batched_ref(
            || s3.clone(),
            |input| {
                for a in input.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, fresh_inserts, remove, get, iter);
criterion_main!(benches);
