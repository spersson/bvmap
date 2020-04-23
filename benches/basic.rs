use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::{thread_rng, Rng};
use slotmap::{DefaultKey, SlotMap};
use stash::{Stash, UniqueStash};
use store::Store;

fn fresh_inserts(c: &mut Criterion) {
    let size = 10_000;
    let s1: Store<usize, usize> = Store::new();
    let s2: Stash<usize, usize> = Stash::new();
    let s3: UniqueStash<usize> = UniqueStash::new();
    let s4: SlotMap<DefaultKey, usize> = SlotMap::new();

    let mut g = c.benchmark_group("Fresh Inserts");
    g.bench_function("Store", |b| {
        b.iter_batched_ref(
            || s1.clone(),
            |i| {
                for a in 0..size {
                    i.insert(a);
                }
            },
            BatchSize::SmallInput,
        );
    });
    g.bench_function("Stash", |b| {
        b.iter_batched_ref(
            || s2.clone(),
            |i| {
                for a in 0..size {
                    i.put(a);
                }
            },
            BatchSize::SmallInput,
        );
    });
    g.bench_function("UniqueStash", |b| {
        b.iter_batched_ref(
            || s3.clone(),
            |i| {
                for a in 0..size {
                    i.put(a);
                }
            },
            BatchSize::SmallInput,
        );
    });
    g.bench_function("SlotMap", |b| {
        b.iter_batched_ref(
            || s4.clone(),
            |i| {
                for a in 0..size {
                    i.insert(a);
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
    let mut s4: SlotMap<DefaultKey, usize> = SlotMap::new();
    let mut s4k = Vec::new();
    for a in 0..size {
        s1.insert(a);
        s2.put(a);
        s3k.push(s3.put(a));
        s4k.push(s4.insert(a));
    }

    let mut g = c.benchmark_group("Remove");
    g.bench_function("Store", |b| {
        b.iter_batched_ref(
            || s1.clone(),
            |i| {
                for _ in 0..size {
                    i.remove(rng.gen_range(0, size));
                }
            },
            BatchSize::SmallInput,
        );
    });
    g.bench_function("Stash", |b| {
        b.iter_batched_ref(
            || s2.clone(),
            |i| {
                for _ in 0..size {
                    i.take(rng.gen_range(0, size));
                }
            },
            BatchSize::SmallInput,
        );
    });
    g.bench_function("UniqueStash", |b| {
        b.iter_batched_ref(
            || s3.clone(),
            |i| {
                for _ in 0..size {
                    i.take(s3k[rng.gen_range(0, size)]);
                }
            },
            BatchSize::SmallInput,
        );
    });
    g.bench_function("SlotMap", |b| {
        b.iter_batched_ref(
            || s4.clone(),
            |i| {
                for _ in 0..size {
                    i.remove(s4k[rng.gen_range(0, size)]);
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
    let mut s4: SlotMap<DefaultKey, usize> = SlotMap::new();
    let mut s4k = Vec::new();
    for a in 0..size {
        s1.insert(a);
        s2.put(a);
        s3k.push(s3.put(a));
        s4k.push(s4.insert(a));
    }
    let mut g = c.benchmark_group("Get");
    g.bench_function("Store", |b| {
        b.iter_batched_ref(
            || s1.clone(),
            |i| {
                for _ in 0..size {
                    black_box(i.get(rng.gen_range(0, size)));
                }
            },
            BatchSize::SmallInput,
        );
    });
    g.bench_function("Stash", |b| {
        b.iter_batched_ref(
            || s2.clone(),
            |i| {
                for _ in 0..size {
                    black_box(i.get(rng.gen_range(0, size)));
                }
            },
            BatchSize::SmallInput,
        );
    });
    g.bench_function("UniqueStash", |b| {
        b.iter_batched_ref(
            || s3.clone(),
            |i| {
                for _ in 0..size {
                    black_box(i.get(s3k[rng.gen_range(0, size)]));
                }
            },
            BatchSize::SmallInput,
        );
    });
    g.bench_function("SlotMap", |b| {
        b.iter_batched_ref(
            || s4.clone(),
            |i| {
                for _ in 0..size {
                    black_box(i.get(s4k[rng.gen_range(0, size)]));
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
    let mut s4: SlotMap<DefaultKey, usize> = SlotMap::new();
    let mut s4k = Vec::new();
    for a in 0..size {
        s1.insert(a);
        s2.put(a);
        s3k.push(s3.put(a));
        s4k.push(s4.insert(a));
    }

    let mut g = c.benchmark_group("Iterate");
    g.bench_function("Store", |b| {
        b.iter_batched_ref(
            || s1.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
    g.bench_function("Stash", |b| {
        b.iter_batched_ref(
            || s2.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
    g.bench_function("UniqueStash", |b| {
        b.iter_batched_ref(
            || s3.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
    g.bench_function("SlotMap", |b| {
        b.iter_batched_ref(
            || s4.clone(),
            |i| {
                for a in i.iter() {
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
        s4.remove(s4k[k]);
    }

    let mut g = c.benchmark_group("Iterate half-full");
    g.bench_function("Store", |b| {
        b.iter_batched_ref(
            || s1.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
    g.bench_function("Stash", |b| {
        b.iter_batched_ref(
            || s2.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
    g.bench_function("UniqueStash", |b| {
        b.iter_batched_ref(
            || s3.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
    g.bench_function("SlotMap", |b| {
        b.iter_batched_ref(
            || s4.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, fresh_inserts, remove, get, iter);
criterion_main!(benches);
