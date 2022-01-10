fn main() {
    println!("Part 1: {}", fight(0));
    println!("Part 2: {}", fight(1));
}

fn fight(damage: u8) -> u16 {
    let mut queue = std::collections::BinaryHeap::from([Priority(
        Player { hp: 50, mana: 500, shield: 0, recharge: 0, spent: 0},
        Boss { hp: 58, poison: 0 },
    )]);

    while let Some(Priority(mut player, mut boss)) = queue.pop() {
        if boss.hp == 0 {
            return player.spent;
        } else if player.hp == 0 {
            continue;
        }

        turn(&mut player, &mut boss, damage);

        for spell in [Player::missile, Player::drain, Player::shield, Player::poison, Player::recharge] {
            let (mut player2, mut boss) = (player.clone(), boss.clone());
            spell(&mut player2, &mut boss);

            if player != player2 && player2.mana > 0 {
                turn(&mut player2, &mut boss, damage);
                boss.attack(&mut player2);
                queue.push(Priority(player2, boss));
            }
        }
    }

    unreachable!()
}

fn turn(player: &mut Player, boss: &mut Boss, damage: u8) {
    player.hp = player.hp.saturating_sub(damage);
    player.shield = player.shield.saturating_sub(1);

    if player.recharge > 0 {
        player.mana += 101;
    }

    player.recharge = player.recharge.saturating_sub(1);

    if boss.poison > 0 {
        boss.hp = boss.hp.saturating_sub(3)
    }

    boss.poison = boss.poison.saturating_sub(1);
}


#[derive(Clone, Copy, Eq, PartialEq)]
struct Player {
    hp: u8,
    mana: u16,
    shield: u8,
    recharge: u8,
    spent: u16,
}

impl Player {
    fn spend(&mut self, mana: u16) {
        self.mana = self.mana.saturating_sub(mana);
        self.spent += mana;
    }

    fn missile(&mut self, boss: &mut Boss) {
        self.spend(53);
        boss.hp = boss.hp.saturating_sub(4);
    }

    fn drain(&mut self, boss: &mut Boss) {
        self.spend(73);
        self.hp += 2;
        boss.hp = boss.hp.saturating_sub(2);
    }

    fn shield(&mut self, _: &mut Boss) {
        if self.shield == 0 {
            self.spend(113);
            self.shield = 6;
        }
    }

    fn poison(&mut self, boss: &mut Boss) {
        if boss.poison == 0 {
            self.spend(173);
            boss.poison = 6;
        }
    }

    fn recharge(&mut self, _: &mut Boss) {
        if self.recharge == 0 {
            self.spend(229);
            self.recharge = 5;
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Boss {
    hp: u8,
    poison: u8,
}

impl Boss {
    fn attack(&self, player: &mut Player) {
        player.hp = player.hp.saturating_sub(9 - if player.shield > 0 { 7 } else { 0 });
    }
}

#[derive(Eq, PartialEq)]
struct Priority(Player, Boss);

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.spent.cmp(&self.0.spent)
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
