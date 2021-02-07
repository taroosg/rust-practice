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
    println!("{} の こうげき！", self.name);
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
    println!("{} に {} の ダメージ！", self.name, damage);
    self.hp -= damage as isize;
    println!("{} の HP：{}\n", self.name, self.hp);

    if self.hp <= 0 {
      println!("{} は たおれた！\n", self.name);
      self.dead = true;
    }
  }
}

fn main() {
  let mut my_pokemon = Pokemon::new("Polygon".to_string(), 500, 100, 75);

  let mut enemy = Vec::<Pokemon>::new();
  // 敵を1-3体生成する
  let enemy_number = rand::thread_rng().gen_range(1, 4);
  for i in 0..enemy_number {
    let adjust_coefficient = create_adjustment_random();
    enemy.push(Pokemon::new(
      "敵".to_string() + &(i + 1).to_string(),
      (100 as f64 * adjust_coefficient).round() as isize,
      (100 as f64 * adjust_coefficient).round() as usize,
      (50 as f64 * adjust_coefficient).round() as usize,
    ))
  }

  loop {
    my_pokemon.attack(&mut enemy[0]);

    // 敵の死を確認
    if enemy[0].dead {
      enemy.remove(0);
    }

    if judge(&my_pokemon, enemy.len()) {
      break;
    }

    for e in &enemy {
      e.attack(&mut my_pokemon);
    }

    if judge(&my_pokemon, enemy.len()) {
      break;
    }
  }
}

fn judge(my_pokemon: &Pokemon, enemy_count: usize) -> bool {
  if my_pokemon.dead {
    println!("{} は めのまえが まっくらに なった！", my_pokemon.name);
    true
  } else if enemy_count == 0 {
    println!("{} は しょうぶに かった！", my_pokemon.name);
    true
  } else {
    false
  }
}

// 調整用：0.9から1.1で乱数
fn create_adjustment_random() -> f64 {
  rand::thread_rng().gen_range(0.9f64, 1.1f64)
}
