struct Pokemon {
  name: String,
  hp: isize,
  power: usize,
  dead: bool,
}

impl Pokemon {
  fn new(name: String, hp: isize, power: usize) -> Self {
    Pokemon {
      name,
      hp,
      power,
      dead: false,
    }
  }

  fn attack(&self, target: &mut Pokemon) {
    println!("{}の攻撃！！", self.name);
    target.damage(self.power)
  }

  fn damage(&mut self, damage: usize) {
    println!("{}に{}のダメージ！！", self.name, damage);
    self.hp -= damage as isize;
    println!("{}のHP：{}\n", self.name, self.hp);

    if self.hp <= 0 {
      println!("{}は倒れた！\n", self.name);
      self.dead = true;
    }
  }
}

fn main() {
  let mut hero = Pokemon::new("Pika".to_string(), 10, 3);

  let mut enemy = Vec::<Pokemon>::new();
  // 敵を3体生成する
  for i in 0..2 {
    enemy.push(Pokemon::new("敵".to_string() + &(i + 1).to_string(), 5, 2))
  }

  loop {
    hero.attack(&mut enemy[0]);

    // 敵の死を確認
    if enemy[0].dead {
      enemy.remove(0);
    }

    if judge(&hero, enemy.len()) {
      break;
    }

    for e in &enemy {
      e.attack(&mut hero);
    }

    if judge(&hero, enemy.len()) {
      break;
    }
  }
}

fn judge(hero: &Pokemon, enemy_count: usize) -> bool {
  if hero.dead {
    println!("{}の敗北！！", hero.name);
    true
  } else if enemy_count == 0 {
    println!("{}の勝利！！", hero.name);
    true
  } else {
    false
  }
}
