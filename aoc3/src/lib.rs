#![feature(iter_array_chunks)]
use std::{io::{Lines, BufReader}, fs::File};

pub fn run(lines: Lines<BufReader<File>>) {
    // // Part One
    // let mut sum = 0;
    // for line in lines {
    //     let line = line.expect("Could not parse line");
    //     sum += rucksack::get_line_priority(line);
    // }
    // println!("Total priority is: {}", sum);

    // Part Two
    let mut sum = 0;
    for group in lines.array_chunks::<3>() {
        let group = group.map(|l| l.unwrap());
        sum += rucksack::get_group_priority(group);
    }
    println!("Total priority is: {}", sum);
}

mod rucksack {
    pub fn get_line_priority(line: String) -> u64 {
        let mut bitstring: u64 = 0; // acts as bit string
        let threshold = line.len() / 2;
        for (i, c) in line.char_indices() {
            let char_bit = char_to_ind(c);
            let mask = (1 as u64).checked_shl(char_bit).unwrap();
            if i < threshold {
                bitstring |= mask;
            } else if (bitstring & mask) > 0 {
                return char_bit as u64;
            }
        }
        panic!("Should have found matching char by end of loop");
    }

    pub fn get_group_priority(group: [String; 3]) -> u64 {
        let mut bitstring = u64::MAX;
        for line in group {
            let mut this_bitstring = 0;
            for c in line.chars() {
                let char_bit = char_to_ind(c);
                this_bitstring |= (1 as u64).checked_shl(char_bit).unwrap();
            }
            bitstring &= this_bitstring;
        }
        (bitstring as f64).log2() as u64
    }

    /// Returns the (1-indexed) index value of the char in the bit string,
    /// with a-z as 1-26 and A-Z being 27-52.
    pub fn char_to_ind(c: char) -> u32 {
        // 'a' is 97, 'A' is 65, so if checked sub works, it's lowercase
        let res = if let Some(diff) = (c as u32).checked_sub('a' as u32) { // lowercase
            diff
        } else { // uppercase
            26 + (c as u32).checked_sub('A' as u32).expect(format!("char {} in line not valid", c).as_str())
        } + 1;

        assert!(res <=53);
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::rucksack::{get_line_priority, char_to_ind};

    #[test]
    #[should_panic]
    fn no_repeat() {
        get_line_priority(String::from("abcd"));
    }

    #[test]
    fn test_chars() {
        assert_eq!(char_to_ind('a'), 1);
        assert_eq!(char_to_ind('z'), 26);
        assert_eq!(char_to_ind('A'), 27);
        assert_eq!(char_to_ind('Z'), 52);
    }
}
