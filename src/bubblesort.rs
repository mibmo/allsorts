use crate::Sorter;

pub struct Bubblesort;

impl<T: Ord> Sorter<T> for Bubblesort {
    fn sort(slice: &mut [T]) {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    swapped = true;
                    slice.swap(i - 1, i);
                }
            }
        }
    }
}

#[test]
fn sort() {
    let mut input = vec![5, 3, 1, 2, 4];
    Bubblesort::sort(&mut input);
    assert_eq!(input, vec![1, 2, 3, 4, 5]);
}
