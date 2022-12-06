use std::iter::Sum;
use std::{collections::BinaryHeap, mem};
use std::cmp::Reverse;

use util::{input_file_path, read_lines};

// PART 1
/// Set `most` and `this_amt` according to if `this_amt` > `most`
// fn new_iter(most: &mut usize, this_amt: &mut usize) {
//     if *this_amt > *most {
//         *most = *this_amt;
//     }
//     *this_amt = 0;
// }

mod elf_heap {
    use super::*;

    pub struct ElfHeap<T: Ord> {
        min_heap: BinaryHeap<Reverse<T>>,
        desired_size: usize,
    }

    impl<T> ElfHeap<T> where T: Ord + Sum {
        pub fn with_size(size: usize) -> Self {
            ElfHeap { min_heap: BinaryHeap::<Reverse<T>>::new(), desired_size: size }
        }

        pub fn push(&mut self, elt: T) {
            self.min_heap.push(Reverse(elt));
            while self.min_heap.len() > self.desired_size {
                self.min_heap.pop();
            }
        }

        fn drain_sorted(self) -> Vec<T> {
            self.min_heap.into_sorted_vec().into_iter().map(|i| i.0).collect()
        }

        pub fn sum(self) -> T {
            self.drain_sorted().into_iter().sum()
        }
    }
}

use elf_heap::ElfHeap;

// PART 2
/// Insert `this_amt` into `min_heap` and pop if necessary to get back to 3 elements.
fn new_iter(elf_heap: &mut ElfHeap<usize>, this_amt: &mut usize) {
    // Create owned var to_push and use mem::swap to avoid clone of `this_amt` value
    let mut to_push = 0; // new `this_amt`
    mem::swap(&mut to_push, this_amt);
    elf_heap.push(to_push);
}

fn main() {
    let lines = read_lines(input_file_path()).expect("Could not parse input file lines");

    // Track elves with 3 highest calorie totals
    let mut elf_heap = ElfHeap::with_size(3);
    // Track sum of lines (calories for one elf)
    let mut this = 0;
    for line in lines {
        let line = line.expect("Could not unwrap line");
        if line.trim().is_empty() {
            new_iter(&mut elf_heap, &mut this);
        } else {
            this += usize::from_str_radix(line.trim(), 10).expect("Could not parse line as usize");
        }
    }
    new_iter(&mut elf_heap, &mut this);

    println!("Sum of top 3 elf calorie counts: {}", elf_heap.sum());
}
