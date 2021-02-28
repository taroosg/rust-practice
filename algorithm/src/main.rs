fn main() {
  println!("Input One Line!");
  let player_input: String = {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_string().parse().unwrap()
  };
  println!("This Email is {:?}.", is_valid_email(player_input));
}

// // 豆腐の一辺から表面積を求める関数
// fn cube_surface_area(num: u32) -> u32 {
//   num.pow(2) * 6
// }

// // きっぷの価格から購入人数を求める関数
// fn weekly_7days_sales(ticket_price: i32) -> i32 {
//   match (250 - ticket_price) * 700 + 150000 {
//     n if n < 0 => 0,
//     n => n,
//   }
// }

// メールアドレスをチェックする関数
// @スタートでない
// スペースなし
// @がひとつのみ
// @のあとに.がある
fn is_valid_email(email: String) -> bool {
  vec![
    !email.starts_with("@"),
    email.split_whitespace().collect::<Vec<&str>>().len() == 1,
    email.split("@").collect::<Vec<&str>>().len() == 2,
    email
      .split("@")
      .collect::<Vec<&str>>()
      .last()
      .unwrap()
      .split("")
      .collect::<Vec<&str>>()
      .contains(&"."),
  ]
  .iter()
  .all(|&x| x == true)
}
