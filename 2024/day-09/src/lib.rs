pub fn none_length(i: usize, fs: &[Option<u64>]) -> usize {
    let mut l = 0;
    for idx in i..fs.len() {
        if fs[idx].is_none() {
            l += 1;
        } else {
            return l;
        }
    }
    l
}

// Vec of indexes and lengths
pub fn spaces(fs: &Vec<Option<u64>>) -> Vec<(usize, usize)> {
    let mut retval = vec![];
    let mut idx = 0;
    while fs[idx].is_some() {
        idx += 1;
    }

    while idx < fs.len() {
        let length = none_length(idx, fs);
        retval.push((idx, length));
        idx += length;
        while idx < fs.len() && fs[idx].is_some() {
            idx += 1;
        }
    }
    retval
}

// Return the start index and length
pub fn id_length(i: usize, fs: &[Option<u64>]) -> (usize, usize) {
    let mut start_idx = i;
    while start_idx > 0 && fs[start_idx] == fs[i] {
        start_idx -= 1;
    }
    match start_idx != 0 {
        true => (start_idx + 1, (i - start_idx)),
        false => (0, i + 1),
    }
}

// return vec of id starting positions and lengths
pub fn id_fs(fs: &[Option<u64>]) -> Vec<(usize, usize)> {
    let mut retval = Vec::new();
    let mut idx = fs.len() - 1;
    while idx > 0 && fs[idx].is_none() {
        idx -= 1;
    }
    while idx > 0 {
        let (start, length) = id_length(idx, fs);
        retval.push((start, length));
        match start > 0 {
            true => {
                idx = start - 1;
            }
            false => idx = start,
        }
        while idx > 0 && fs[idx].is_none() {
            idx -= 1;
        }
    }
    retval
}

pub fn make_fs(input: &str) -> Vec<Option<u64>> {
    // let expanded = "00...111...2...333.44.5555.6666.777.888899";
    let mut id = 0_u64;
    let mut fs: Vec<Option<u64>> = Vec::new();
    let mut is_file = true;

    for c in input.trim().chars() {
        let length = c
            .to_digit(10)
            // .expect(format!("{} should be a digit", c).as_str());
            .unwrap_or_else(|| panic!("[{}] should be a digit", c));
        for _ in 0..length {
            match is_file {
                true => fs.push(Some(id)),
                false => fs.push(None),
            }
        }
        if is_file {
            id += 1;
        }
        is_file = !is_file;
    }
    fs
}
pub fn process_part2(input: &str) -> u64 {
    // Rules2: 00992111777.44.333....5555.6666.....8888..
    let mut fs = make_fs(input);

    let mut fs_free = spaces(&fs);
    let ids = id_fs(&fs);
    // For all the filesystem blocks of ids see if we can move it
    // and if we can do.
    for (id, length) in ids.iter() {
        let mut swapped = false;
        for free_id in 0..fs_free.len() {
            let (free, freel) = fs_free[free_id];

            if !swapped && free < *id && freel >= *length {
                for delta_idx in 0..(*length as u64) {
                    fs[free + delta_idx as usize] = fs[id + delta_idx as usize];
                    fs[id + delta_idx as usize] = None;
                }
                fs_free[free_id] = (free + length, freel - length);
                swapped = true;
            }
        }
    }
    checksum(fs)
}

pub fn process_part1(input: &str) -> u64 {
    let mut fs: Vec<Option<u64>> = make_fs(input);

    let mut left = 0;
    let mut right = fs.len() - 1;
    while fs[left].is_some() {
        left += 1;
    }
    while fs[right].is_none() {
        right -= 1;
    }
    while left < right {
        if fs[left].is_none() && fs[right].is_some() {
            fs[left] = fs[right];
            fs[right] = None;
            left += 1;
            right -= 1;
        } else {
            while fs[left].is_some() {
                left += 1;
            }
            while fs[right].is_none() {
                right -= 1;
            }
        }
    }

    checksum(fs)
}

pub fn checksum(fs: Vec<Option<u64>>) -> u64 {
    fs.iter()
        .enumerate()
        .map(|(i, optc)| match optc {
            Some(x) => (i as u64) * x,
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 1928);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 2858);
    }
}
