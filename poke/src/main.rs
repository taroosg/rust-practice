use rand::prelude::*;

struct Pokemon {
  name: String,
  hp: isize,
  power: usize,
  defence: usize,
  dead: bool,
}

impl Pokemon {
  fn new(name: String, hp: isize, power: usize, defence: usize) -> Self {
    Pokemon {
      name,
      hp,
      power,
      defence,
      dead: false,
    }
  }

  fn attack(&self, target: &mut Pokemon) {
    println!("{}の攻撃！！", self.name);
    target.damage(self.power, target.defence)
  }

  fn damage(&mut self, power: usize, defence: usize) {
    let param = power as i32 - defence as i32;
    let damage: u32;
    if param < 0 {
      damage = 0;
    } else {
      let adjust_coefficient = create_adjustment_random();
      damage = (param as f64 * adjust_coefficient).round() as u32;
    }
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
  let mut hero = Pokemon::new("Polygon".to_string(), 500, 100, 75);

  let mut enemy = Vec::<Pokemon>::new();
  // 敵を3体生成する
  for i in 0..3 {
    enemy.push(Pokemon::new(
      "敵".to_string() + &(i + 1).to_string(),
      100,
      100,
      50,
    ))
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

// 調整用：0.9から1.1で乱数
fn create_adjustment_random() -> f64 {
  let mut rng = rand::thread_rng();
  rng.gen_range(0.9f64, 1.1f64)
}
