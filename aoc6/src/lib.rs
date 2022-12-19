use std::collections::HashMap;

fn get_start_by_distinct(line: String, num_distinct: usize) -> usize {
    let mut char_map = HashMap::<char, usize>::new();
    let chars = line.chars().collect::<Vec<char>>();
    for (i, c) in chars.iter().enumerate() {
        // Add one to char_map[c]
        char_map.entry(*c).and_modify(|x| *x += 1).or_insert(1);

        // Check for duplicates
        if i >= num_distinct - 1 && char_map.values().into_iter().all(|count| *count == 1) {
            return i;
        }

        // If we're far enough, delete the character that has expired
        // from consideration for the distinct sequence
        if i >= num_distinct {
            let four_back = chars[i - num_distinct];
            char_map.entry(four_back).and_modify(|count| *count -= 1);
            if *char_map.get(&four_back).unwrap() == 0 {
                char_map.remove(&four_back);
            }
        }
    }
    panic!()
}

pub fn run1<T>(mut lines: T)
where
    T: Iterator<Item = String>,
{
    let line = lines.next().unwrap();
    assert!(lines.next().is_none());

    println!("{}", get_start_by_distinct(line, 4));
}

pub fn run2<T>(mut lines: T)
where
    T: Iterator<Item = String>,
{
    let line = lines.next().unwrap();
    assert!(lines.next().is_none());

    println!("{}", get_start_by_distinct(line, 14));
}
