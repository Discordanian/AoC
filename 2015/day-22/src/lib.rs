pub fn process_part1(input: &str) -> String {
    let (boss_hp, boss_damage) = parse_boss(input);
    let min_spent = min_mana_to_win(boss_hp, boss_damage, false);
    min_spent.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (boss_hp, boss_damage) = parse_boss(input);
    let min_spent = min_mana_to_win(boss_hp, boss_damage, true);
    min_spent.to_string()
}

fn parse_boss(input: &str) -> (i32, i32) {
    let mut hp = 0;
    let mut dmg = 0;
    for line in input.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("Hit Points:") {
            hp = rest.trim().parse().unwrap_or(0);
        } else if let Some(rest) = line.strip_prefix("Damage:") {
            dmg = rest.trim().parse().unwrap_or(0);
        }
    }
    (hp, dmg)
}

#[derive(Clone, Copy)]
struct State {
    hp: i32,
    mana: i32,
    boss_hp: i32,
    shield: i32,
    poison: i32,
    recharge: i32,
    spent: i32,
}

/// Apply start-of-turn effects (Shield timer, Poison damage, Recharge mana).
/// Order matches the puzzle worked examples: Shield, then Poison, then Recharge.
fn apply_effects(s: &mut State) -> bool {
    if s.shield > 0 {
        s.shield -= 1;
    }
    if s.poison > 0 {
        s.boss_hp -= 3;
        s.poison -= 1;
    }
    if s.recharge > 0 {
        s.mana += 101;
        s.recharge -= 1;
    }
    s.boss_hp <= 0
}

fn armor_after_effects(s: &State) -> i32 {
    if s.shield > 0 {
        7
    } else {
        0
    }
}

fn min_mana_to_win(boss_hp: i32, boss_damage: i32, hard: bool) -> i32 {
    let mut best = i32::MAX;
    let start = State {
        hp: 50,
        mana: 500,
        boss_hp,
        shield: 0,
        poison: 0,
        recharge: 0,
        spent: 0,
    };
    search(start, boss_damage, hard, &mut best);
    best
}

fn search(mut s: State, boss_damage: i32, hard: bool, best: &mut i32) {
    if s.spent >= *best {
        return;
    }

    // Hard mode: lose 1 HP at the start of each player turn, before any effects
    if hard {
        s.hp -= 1;
        if s.hp <= 0 {
            return;
        }
    }

    // Player turn: effects
    if apply_effects(&mut s) {
        *best = (*best).min(s.spent);
        return;
    }
    if s.hp <= 0 {
        return;
    }

    // Magic Missile (53)
    if s.mana >= 53 {
        let mut t = s;
        t.mana -= 53;
        t.spent += 53;
        t.boss_hp -= 4;
        continue_after_cast(t, boss_damage, hard, best);
    }
    // Drain (73)
    if s.mana >= 73 {
        let mut t = s;
        t.mana -= 73;
        t.spent += 73;
        t.boss_hp -= 2;
        t.hp += 2;
        continue_after_cast(t, boss_damage, hard, best);
    }
    // Shield (113), 6 turns
    if s.mana >= 113 && s.shield == 0 {
        let mut t = s;
        t.mana -= 113;
        t.spent += 113;
        t.shield = 6;
        continue_after_cast(t, boss_damage, hard, best);
    }
    // Poison (173)
    if s.mana >= 173 && s.poison == 0 {
        let mut t = s;
        t.mana -= 173;
        t.spent += 173;
        t.poison = 6;
        continue_after_cast(t, boss_damage, hard, best);
    }
    // Recharge (229)
    if s.mana >= 229 && s.recharge == 0 {
        let mut t = s;
        t.mana -= 229;
        t.spent += 229;
        t.recharge = 5;
        continue_after_cast(t, boss_damage, hard, best);
    }
}

fn continue_after_cast(mut s: State, boss_damage: i32, hard: bool, best: &mut i32) {
    if s.boss_hp <= 0 {
        *best = (*best).min(s.spent);
        return;
    }

    // Boss turn: effects
    if apply_effects(&mut s) {
        *best = (*best).min(s.spent);
        return;
    }

    let armor = armor_after_effects(&s);
    let hit = (boss_damage - armor).max(1);
    s.hp -= hit;
    if s.hp <= 0 {
        return;
    }

    search(s, boss_damage, hard, best);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_puzzle_input() {
        let input = include_str!("../input.txt");
        assert_eq!(process_part1(input), "1269");
    }

    #[test]
    fn part2_puzzle_input() {
        let input = include_str!("../input.txt");
        assert_eq!(process_part2(input), "1309");
    }
}
