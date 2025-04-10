/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::fmt::Debug;

fn sort<T>(array: &mut [T]) where T: PartialOrd + Copy{
	quickSort(array, 0, array.len() as u32 - 1);
}

fn quickSort<T>(array: &mut [T], left: u32, right: u32) where T: PartialOrd + Copy{
	if left >= right {
        return;
    }
    
    let x = array[(left + right) as usize / 2];
    let mut l = left;
    let mut r = right;
    while l < r {
        while array[r as usize] > x {
            r -= 1;
        }
        while array[l as usize] < x {
            l += 1;
        }
        if l < r {
            array.swap(l as usize, r as usize);
        }
    }
    quickSort(array, left, r);
    quickSort(array, r + 1, right);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}