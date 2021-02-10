use rand::prelude::*;

struct Pokemon {
  name: String,
  hp: isize,
  power: usize,
  defence: usize,
  dead: bool,
  moves: [Move; 4],
}

#[derive(Debug)]
struct Move {
  name: String,
  pwr: u32,
  ass: u32,
}

impl Move {
  fn new(name: String, pwr: u32, ass: u32) -> Self {
    Move { name, pwr, ass }
  }
}

impl Pokemon {
  fn new(name: String, hp: isize, power: usize, defence: usize, moves: [Move; 4]) -> Self {
    Pokemon {
      name,
      hp,
      power,
      defence,
      dead: false,
      moves: moves,
    }
  }

  fn attack(&self, target: &mut Pokemon, my_move: u32) {
    println!("{} の {}！", self.name, self.moves[my_move as usize].name);
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
  // 自分のわざ
  let my_moves = [
    Move::new("たいあたり".to_string(), 30, 95),
    Move::new("10まんボルト".to_string(), 95, 100),
    Move::new("トライアタック".to_string(), 85, 100),
    Move::new("はかいこうせん".to_string(), 150, 95),
  ];
  // 自分のポケモン
  let mut my_pokemon = Pokemon::new("Polygon".to_string(), 500, 100, 75, my_moves);

  // 敵の生成
  let mut enemy = Vec::<Pokemon>::new();
  let enemy_number = rand::thread_rng().gen_range(1, 4);
  for i in 0..enemy_number {
    let adjust_coefficient = create_adjustment_random();
    let enemy_moves = [
      Move::new("たいあたり".to_string(), 30, 95),
      Move::new("ひっかく".to_string(), 40, 100),
      Move::new("メガトンパンチ".to_string(), 80, 70),
      Move::new("メガトンキック".to_string(), 120, 60),
    ];
    enemy.push(Pokemon::new(
      "敵".to_string() + &(i + 1).to_string(),
      (100 as f64 * adjust_coefficient).round() as isize,
      (100 as f64 * adjust_coefficient).round() as usize,
      (50 as f64 * adjust_coefficient).round() as usize,
      enemy_moves,
    ))
  }

  loop {
    // コマンドを受け取る
    let my_move = get_command(&my_pokemon.moves);
    println!("{}", my_move);

    // こうげき
    my_pokemon.attack(&mut enemy[0], my_move);

    // 敵の死を確認
    if enemy[0].dead {
      enemy.remove(0);
    }

    if judge(&my_pokemon, enemy.len()) {
      break;
    }

    // 敵のこうげき
    for e in &enemy {
      let enemy_move = rand::thread_rng().gen_range(0, 4);
      e.attack(&mut my_pokemon, enemy_move);
    }

    if judge(&my_pokemon, enemy.len()) {
      break;
    }
  }
}

fn get_command(my_moves: &[Move; 4]) -> u32 {
  let my_move: u32;
  loop {
    println!(
      "command?? 0:{}, 1:{}, 2:{}, 3:{}",
      my_moves[0].name, my_moves[1].name, my_moves[2].name, my_moves[3].name
    );
    let player_command: u32 = {
      let mut s = String::new();
      std::io::stdin().read_line(&mut s).unwrap();
      s.trim_end().to_string().parse().unwrap()
    };
    println!("player command is {}", player_command);
    match player_command {
      0 | 1 | 2 | 3 => {
        my_move = player_command;
        break;
      }
      _ => println!("invailed command {}. please try again.", player_command),
    };
  }
  my_move
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
