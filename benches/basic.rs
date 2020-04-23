use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::{thread_rng, Rng};
use slotmap::{DefaultKey, SlotMap, HopSlotMap};
use stash::{Stash, UniqueStash};
use store::Store;

fn inserts(c: &mut Criterion) {
    let size = 10_000;
    let s1: Store<usize, usize> = Store::new();
    let s2: Stash<usize, usize> = Stash::new();
    let s3: UniqueStash<usize> = UniqueStash::new();
    let s4: SlotMap<DefaultKey, usize> = SlotMap::new();
    let s5: HopSlotMap<DefaultKey, usize> = HopSlotMap::new();

    let mut g = c.benchmark_group("Inserts");
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
    g.bench_function("HopSlotMap", |b| {
        b.iter_batched_ref(
            || s5.clone(),
            |i| {
                for a in 0..size {
                    i.insert(a);
                }
            },
            BatchSize::SmallInput,
        );
    });
}

fn reinserts(c: &mut Criterion) {
    let size = 10_000;
    let mut s1: Store<usize, usize> = Store::new();
    let mut s2: Stash<usize, usize> = Stash::new();
    let mut s3: UniqueStash<usize> = UniqueStash::new();
    let mut s3k = Vec::new();
    let mut s4: SlotMap<DefaultKey, usize> = SlotMap::new();
    let mut s4k = Vec::new();
    let mut s5: HopSlotMap<DefaultKey, usize> = HopSlotMap::new();
    let mut s5k = Vec::new();

    for a in 0..size {
        s1.insert(a);
        s2.put(a);
        s3k.push(s3.put(a));
        s4k.push(s4.insert(a));
        s5k.push(s5.insert(a));
    }
    for a in 0..size {
        s1.remove(a);
        s2.take(a);
        s3.take(s3k[a]);
        s4.remove(s4k[a]);
        s5.remove(s5k[a]);
    }
    let mut g = c.benchmark_group("Re-inserts");
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
    g.bench_function("HopSlotMap", |b| {
        b.iter_batched_ref(
            || s5.clone(),
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
    let mut s5: HopSlotMap<DefaultKey, usize> = HopSlotMap::new();
    let mut s5k = Vec::new();

    for a in 0..size {
        s1.insert(a);
        s2.put(a);
        s3k.push(s3.put(a));
        s4k.push(s4.insert(a));
        s5k.push(s5.insert(a));
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
    g.bench_function("HopSlotMap", |b| {
        b.iter_batched_ref(
            || s5.clone(),
            |i| {
                for _ in 0..size {
                    i.remove(s5k[rng.gen_range(0, size)]);
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
    let mut s5: HopSlotMap<DefaultKey, usize> = HopSlotMap::new();
    let mut s5k = Vec::new();

    for a in 0..size {
        s1.insert(a);
        s2.put(a);
        s3k.push(s3.put(a));
        s4k.push(s4.insert(a));
        s5k.push(s5.insert(a));
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
    g.bench_function("HopSlotMap", |b| {
        b.iter_batched_ref(
            || s5.clone(),
            |i| {
                for _ in 0..size {
                    black_box(i.get(s5k[rng.gen_range(0, size)]));
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
    let mut s1k = Vec::new();
    let mut s2: Stash<usize, usize> = Stash::new();
    let mut s2k = Vec::new();
    let mut s3: UniqueStash<usize> = UniqueStash::new();
    let mut s3k = Vec::new();
    let mut s4: SlotMap<DefaultKey, usize> = SlotMap::new();
    let mut s4k = Vec::new();
    let mut s5: HopSlotMap<DefaultKey, usize> = HopSlotMap::new();
    let mut s5k = Vec::new();

    for a in 0..size {
        s1k.push(s1.insert(a));
        s2k.push(s2.put(a));
        s3k.push(s3.put(a));
        s4k.push(s4.insert(a));
        s5k.push(s5.insert(a));
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
    g.bench_function("HopSlotMap", |b| {
        b.iter_batched_ref(
            || s5.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
    g.finish();

    for subset in ((size / 2)..size).rev() {
        let k = rng.gen_range(0, subset);
        s1.remove(s1k[k]);
        s1k.swap_remove(k);
        s2.take(s2k[k]);
        s2k.swap_remove(k);
        s3.take(s3k[k]);
        s3k.swap_remove(k);
        s4.remove(s4k[k]);
        s4k.swap_remove(k);
        s5.remove(s5k[k]);
        s5k.swap_remove(k);
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
    g.bench_function("HopSlotMap", |b| {
        b.iter_batched_ref(
            || s5.clone(),
            |i| {
                for a in i.iter() {
                    black_box(a);
                }
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, inserts, reinserts, remove, get, iter);
criterion_main!(benches);
