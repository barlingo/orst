pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord + Copy;
}

mod bubblesort;
mod heapsort;
mod insertionsort;
mod mergesort;
mod quicksort;
mod radixsort;
mod selectionsort;

pub use bubblesort::BubbleSort;
pub use heapsort::HeapSort;
pub use insertionsort::InsertionSort;
pub use mergesort::MergeSort;
pub use quicksort::QuickSort;
pub use radixsort::Bytify;
pub use radixsort::RadixSort;
pub use selectionsort::SelectionSort;

pub struct StdSorter;
impl<T> Sorter<T> for StdSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord + Copy,
    {
        slice.sort();
    }
}

pub struct StdUnstableSorter;
impl<T> Sorter<T> for StdUnstableSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort_unstable();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 3, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }

    #[test]
    fn stdunstable_works() {
        let mut things = vec![4, 2, 3, 1];
        StdUnstableSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
}
