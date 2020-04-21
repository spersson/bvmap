#![feature(untagged_unions)]
use smallbitvec::SmallBitVec;
use std::marker::PhantomData;
use std::mem::{needs_drop, replace, ManuallyDrop};

union Slot<V> {
    value: ManuallyDrop<V>,
    next_free: usize,
}

#[derive(Default)]
pub struct Store<K, V> {
    next_free: usize,
    bitvec: SmallBitVec,
    vec: Vec<Slot<V>>,
    marker: PhantomData<fn(K) -> K>,
}

impl<K: Into<usize> + From<usize>, V> Store<K, V> {
    pub fn new() -> Store<K, V> {
        Store {
            next_free: 0,
            bitvec: SmallBitVec::new(),
            vec: Vec::new(),
            marker: PhantomData,
        }
    }

    pub fn insert(&mut self, v: V) -> K {
        let next_free = self.next_free;
        if next_free == self.vec.len() {
            self.vec.push(Slot {
                value: ManuallyDrop::new(v),
            });
            self.bitvec.push(true);
            self.next_free += 1;
        } else {
            let slot = replace(
                &mut self.vec[next_free],
                Slot {
                    value: ManuallyDrop::new(v),
                },
            );
            self.next_free = unsafe { slot.next_free };
            self.bitvec.set(next_free, true);
        }
        K::from(next_free)
    }

    pub fn get(&self, k: K) -> Option<&V> {
        let k = k.into();
        self.bitvec.get(k).and_then(|o| {
            if o {
                Some(unsafe { &*self.vec[k].value })
            } else {
                None
            }
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &V> {
        Iter {
            inner: self
                .vec
                .iter()
                .zip(self.bitvec.iter())
                .filter_map(|(slot, occupied)| {
                    if occupied {
                        Some(unsafe { &*slot.value })
                    } else {
                        None
                    }
                }),
        }
    }

    pub fn remove(&mut self, k: K) -> Option<V> {
        let k = k.into();
        if self.bitvec.get(k)? {
            self.bitvec.set(k, false);
            let next_free = replace(&mut self.next_free, k);
            let slot = replace(&mut self.vec[k], Slot { next_free });
            Some(ManuallyDrop::into_inner(unsafe { slot.value }))
        } else {
            None
        }
    }
}

impl<K, V: Clone> Clone for Store<K, V> {
    fn clone(&self) -> Self {
        let mut vec: Vec<Slot<V>> = Vec::with_capacity(self.vec.len());
        for (slot, occupied) in self.vec.iter().zip(self.bitvec.iter()) {
            vec.push(if occupied {
                Slot {
                    value: unsafe { &slot.value }.clone(),
                }
            } else {
                Slot {
                    next_free: unsafe { slot.next_free },
                }
            });
        }
        Store {
            vec,
            bitvec: self.bitvec.clone(),
            next_free: self.next_free,
            marker: PhantomData,
        }
    }
}

impl<K, V> Drop for Store<K, V> {
    fn drop(&mut self) {
        if needs_drop::<V>() {
            for (slot, occupied) in self.vec.drain(..).zip(self.bitvec.iter()) {
                if occupied {
                    let _ = ManuallyDrop::into_inner(unsafe { slot.value });
                }
            }
        }
    }
}

struct Iter<I> {
    inner: I,
}

impl<'a, V: 'a, I: Iterator<Item = &'a V>> Iterator for Iter<I> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::Store;
    #[test]
    fn basic() {
        let mut store: Store<usize, usize> = Store::new();
        let a1 = store.insert(11);
        let a2 = store.insert(12);
        assert_eq!(store.get(a1), Some(&11));
        assert_eq!(store.get(34), None);
        assert_eq!(store.remove(a2), Some(12));
        assert_eq!(store.get(a2), None);
    }
}
