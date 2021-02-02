extern crate rand;
use rand::Rng;

fn main() {
  let hand = ["✊", "✌", "✋"];
  let mut record = vec![];

  println!("Janken Game!!!");
  println!("0:✊ 1:✌ 2:✋");

  loop {
    let player_hand: u32 = {
      let mut s = String::new();
      std::io::stdin().read_line(&mut s).unwrap();
      s.trim_end().to_string().parse().unwrap()
    };

    match player_hand {
      0 | 1 | 2 => {
        let server_hand: u32 = rand::thread_rng().gen_range(0, 3);
        let result = get_janken_result(player_hand, server_hand);
        record.push(result);
        println!("player hand is {}", hand[player_hand as usize]);
        println!("server hand is {}", hand[server_hand as usize]);
        // このあたり所有権で引っかかる
        // println!("the result is {}", result);
        println!("the record is {:?}", record);

        // じゃんけん3回勝負
        match record.len() {
          3 => break,
          _ => continue,
        }
      }
      _ => println!("invailed hand {}. please try again.", player_hand),
    }
  }
}

fn get_janken_result(player_hand: u32, server_hand: u32) -> String {
  let result_sheet = [
    ["Draw", "Win", "Lose"],
    ["Lose", "Draw", "Win"],
    ["Win", "Lose", "Draw"],
  ];
  return result_sheet[player_hand as usize][server_hand as usize].to_string();
}
