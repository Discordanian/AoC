enum Instruction<'a> {
    Remove(&'a str),
    Add(Lens<'a>),
}

impl<'a> Instruction<'a> {
    fn new(s: &'a str) -> Self {
        if let Some(label) = s.strip_suffix('-') {
            Self::Remove(label)
        } else {
            let (label, focal) = s.split_once('=').unwrap();
            let focal = focal.parse().unwrap();
            let lens = Lens { label, focal };
            Self::Add(lens)
        }
    }
}
struct Lens<'a> {
    label: &'a str,
    focal: u8,
}

pub fn hash1(input: &str) -> u32 {
    input.trim_end().chars().fold(0_u32, |mut total, c| {
        total += c as u32;
        total *= 17;
        total % 256
    })
}

pub fn process_part1(input: &str) -> u32 {
    input.split(',').map(hash1).sum()
}

pub fn process_part2(input: &str) -> usize {
    const BOX: Vec<Lens> = Vec::new();
    let mut boxes = [BOX; 256];

    for instr in input.trim_end().split(',').map(Instruction::new) {
        match instr {
            Instruction::Remove(label) => {
                let hash = hash1(label);
                boxes[hash as usize].retain(|item| item.label != label);
            }
            Instruction::Add(lens) => {
                let hash = hash1(lens.label);
                let lenses = &mut boxes[hash as usize];
                if let Some(old) = lenses.iter_mut().find(|item| lens.label == item.label) {
                    // update focal length of lens with this label
                    old.focal = lens.focal;
                } else {
                    // add lens to end of box
                    lenses.push(lens);
                }
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(box_idx, lenses)| {
            let box_focusing_power: usize = lenses
                .into_iter()
                .enumerate()
                .map(|(lens_idx, lens)| (box_idx + 1) * (lens_idx + 1) * lens.focal as usize)
                .sum();
            box_focusing_power
        })
        .sum()
}

pub fn process_part2k(input: &str) -> u32 {
    let instructions: Vec<&str> = input.split(',').collect();
    let mut boxes: Vec<Vec<(String, u32)>> = (0..256).map(|_| vec![]).collect();

    for instruction in instructions {
        if instruction.contains('=') {
            let parts: Vec<&str> = instruction.split('=').collect();
            let label = parts[0];
            let lens: u32 = parts[1].parse().unwrap();
            let box_id = hash1(label) as usize;
            boxes[box_id].push((label.to_string(), lens));
        } else {
            let parts: Vec<&str> = instruction.split('-').collect();
            let label = parts[0];
            let box_id = hash1(label) as usize;
            if let Some(index) = boxes[box_id].iter().position(|x| x.0 == *label.to_string()) {
                boxes[box_id].remove(index);
            }
        }
    }

    for (x, b) in boxes.clone().into_iter().enumerate().take(5) {}
    boxes.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 1320);
    }

    #[test]
    fn part1a_works() {
        let result = hash1("rn=1");
        assert_eq!(result, 30);
    }

    #[test]
    fn part1b_works() {
        let result = hash1("cm-");
        assert_eq!(result, 253);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 145);
    }
}
