use crate::Sorter;

pub struct Insertionsort;

impl<T: Ord> Sorter<T> for Insertionsort {
    fn sort(slice: &mut [T]) {
        let mut i = 1;
        while i < slice.len() {
            let mut j = i;
            while j > 0 && slice[j-1] > slice[j] {
                slice.swap(j, j-1);
                j -= 1;
            }
            i += 1;
        }
    }
}

#[test]
fn sort() {
    let mut input = vec![5, 3, 1, 2, 4];
    Insertionsort::sort(&mut input);
    assert_eq!(input, vec![1, 2, 3, 4, 5]);
}
