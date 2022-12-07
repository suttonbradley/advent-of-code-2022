/*
--- Day 1: Calorie Counting ---
Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to deliver presents on Christmas. For that, their favorite snack is a special type of star fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.

To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).

The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end up with the following list:

1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
This list represents the Calories of the food carried by five Elves:

The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
The second Elf is carrying one food item with 4000 Calories.
The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
The fifth Elf is carrying one food item with 10000 Calories.
In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).

Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

--- Part Two ---
By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.

To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.

In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories). The sum of the Calories carried by these three elves is 45000.

Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
*/
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
