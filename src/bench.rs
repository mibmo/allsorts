#![feature(is_sorted)]

use allsorts::*;
use rand::prelude::*;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};
use std::sync::{
    atomic::{AtomicUsize, Ordering as AtomicOrdering},
    Arc,
};

#[derive(Clone)]
struct SortEvaluator<T> {
    value: T,
    cmps: Arc<AtomicUsize>,
}

impl<T: Display> Display for SortEvaluator<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl<T: Debug> Debug for SortEvaluator<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<T> SortEvaluator<T> {
    #[inline(always)]
    fn wrap(value: T, cmps: Arc<AtomicUsize>) -> Self {
        Self { value, cmps }
    }
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.fetch_add(1, AtomicOrdering::Relaxed);
        self.value.eq(&other.value)
    }
}
impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmps.fetch_add(1, AtomicOrdering::Relaxed);
        self.value.partial_cmp(&other.value)
    }
}
impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

fn main() {
    let cmp_counter = Arc::new(AtomicUsize::new(0));
    let mut rng = thread_rng();

    println!("algorithm n comparisons runtime"); // print headers for data collection
    for n in &[
        0, 1, 5, 100, 1000, 5000, 10_000,
    ] {
        let mut values: Vec<SortEvaluator<usize>> = (0..*n)
            .map(|_| SortEvaluator::wrap(rng.gen(), Arc::clone(&cmp_counter)))
            .collect();

        for _ in 0..10 {
            values.shuffle(&mut rng);

            bench_sort::<_, StdSorter>(&mut values.clone(), cmp_counter.clone());
            bench_sort::<_, Bubblesort>(&mut values.clone(), cmp_counter.clone());
            bench_sort::<_, Insertionsort>(&mut values.clone(), cmp_counter.clone());
        }
    }
}

fn bench_sort<T: Ord + Clone, S: Sorter<T>>(values: &mut [T], counter: Arc<AtomicUsize>) {
    counter.store(0, AtomicOrdering::Relaxed);
    let sorter_name = std::any::type_name::<S>()
        .split("::")
        .last()
        .expect("could not find type name");

    let start = std::time::Instant::now();
    S::sort(values);
    let elapsed = start.elapsed();

    println!(
        "{alg:15} {n:8} {cmps:12} {took:10}",
        alg = sorter_name,
        n = values.len(),
        cmps = counter.load(AtomicOrdering::Relaxed),
        took = elapsed.as_secs_f64()
    );
}
