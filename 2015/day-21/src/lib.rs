#[derive(Clone, Copy)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

const WEAPONS: [Item; 5] = [
    Item { cost: 8, damage: 4, armor: 0 },
    Item { cost: 10, damage: 5, armor: 0 },
    Item { cost: 25, damage: 6, armor: 0 },
    Item { cost: 40, damage: 7, armor: 0 },
    Item { cost: 74, damage: 8, armor: 0 },
];

const ARMORS: [Item; 5] = [
    Item { cost: 13, damage: 0, armor: 1 },
    Item { cost: 31, damage: 0, armor: 2 },
    Item { cost: 53, damage: 0, armor: 3 },
    Item { cost: 75, damage: 0, armor: 4 },
    Item { cost: 102, damage: 0, armor: 5 },
];

const RINGS: [Item; 6] = [
    Item { cost: 25, damage: 1, armor: 0 },
    Item { cost: 50, damage: 2, armor: 0 },
    Item { cost: 100, damage: 3, armor: 0 },
    Item { cost: 20, damage: 0, armor: 1 },
    Item { cost: 40, damage: 0, armor: 2 },
    Item { cost: 80, damage: 0, armor: 3 },
];

const NO_ARMOR: Item = Item {
    cost: 0,
    damage: 0,
    armor: 0,
};

fn parse_boss(input: &str) -> (i32, i32, i32) {
    let mut hp = 0;
    let mut damage = 0;
    let mut armor = 0;
    for line in input.lines() {
        let line = line.trim();
        if let Some(v) = line.strip_prefix("Hit Points: ") {
            hp = v.parse().expect("boss hp");
        } else if let Some(v) = line.strip_prefix("Damage: ") {
            damage = v.parse().expect("boss damage");
        } else if let Some(v) = line.strip_prefix("Armor: ") {
            armor = v.parse().expect("boss armor");
        }
    }
    (hp, damage, armor)
}

fn player_wins(
    player_hp: i32,
    player_damage: i32,
    player_armor: i32,
    boss_hp: i32,
    boss_damage: i32,
    boss_armor: i32,
) -> bool {
    let mut php = player_hp;
    let mut bhp = boss_hp;
    loop {
        bhp -= (player_damage - boss_armor).max(1);
        if bhp <= 0 {
            return true;
        }
        php -= (boss_damage - player_armor).max(1);
        if php <= 0 {
            return false;
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let (boss_hp, boss_damage, boss_armor) = parse_boss(input);
    const PLAYER_HP: i32 = 100;

    let mut best: Option<i32> = None;

    for weapon in WEAPONS {
        let armor_choices: [&Item; 6] = [
            &NO_ARMOR,
            &ARMORS[0],
            &ARMORS[1],
            &ARMORS[2],
            &ARMORS[3],
            &ARMORS[4],
        ];
        for armor in armor_choices {
            // 0 rings
            try_loadout(
                weapon,
                *armor,
                NO_ARMOR,
                NO_ARMOR,
                PLAYER_HP,
                boss_hp,
                boss_damage,
                boss_armor,
                &mut best,
            );
            // 1 ring
            for r in RINGS {
                try_loadout(
                    weapon,
                    *armor,
                    r,
                    NO_ARMOR,
                    PLAYER_HP,
                    boss_hp,
                    boss_damage,
                    boss_armor,
                    &mut best,
                );
            }
            // 2 distinct rings
            for i in 0..RINGS.len() {
                for j in (i + 1)..RINGS.len() {
                    try_loadout(
                        weapon,
                        *armor,
                        RINGS[i],
                        RINGS[j],
                        PLAYER_HP,
                        boss_hp,
                        boss_damage,
                        boss_armor,
                        &mut best,
                    );
                }
            }
        }
    }

    best.expect("at least one winning loadout")
        .to_string()
}

fn try_loadout(
    weapon: Item,
    armor: Item,
    ring1: Item,
    ring2: Item,
    player_hp: i32,
    boss_hp: i32,
    boss_damage: i32,
    boss_armor: i32,
    best: &mut Option<i32>,
) {
    let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
    let damage = weapon.damage + armor.damage + ring1.damage + ring2.damage;
    let armor_stat = weapon.armor + armor.armor + ring1.armor + ring2.armor;

    if player_wins(
        player_hp,
        damage,
        armor_stat,
        boss_hp,
        boss_damage,
        boss_armor,
    ) {
        let new_best = match *best {
            None => cost,
            Some(c) => c.min(cost),
        };
        *best = Some(new_best);
    }
}

pub fn process_part2(input: &str) -> String {
    input.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_fight_player_wins() {
        // 8 HP, 5 damage, 5 armor vs boss 12 HP, 7 damage, 2 armor
        assert!(player_wins(8, 5, 5, 12, 7, 2));
    }

    #[test]
    fn part1_sample_input() {
        let input = "Hit Points: 103\nDamage: 9\nArmor: 2\n";
        assert_eq!(process_part1(input), "121");
    }
}
