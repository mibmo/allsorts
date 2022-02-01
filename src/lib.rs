pub trait Sorter<T: Ord> {
    fn sort(slice: &mut [T]);
}

mod bubblesort;
mod insertionsort;

pub use bubblesort::Bubblesort;
pub use insertionsort::Insertionsort;

pub struct StdSorter;

impl<T: Ord> Sorter<T> for StdSorter {
    fn sort(slice: &mut [T]) {
        slice.sort();
    }
}

#[cfg(test)]
mod tests {
    use crate::Sorter;

    #[test]
    fn std_works() {
        use crate::StdSorter;

        let mut input = vec![5, 3, 1, 2, 4];
        StdSorter::sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4, 5]);
    }
}
