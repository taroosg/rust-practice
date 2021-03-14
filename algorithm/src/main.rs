fn main() {
  println!("Input One Line!");
  let player_input: isize = {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_string().parse().unwrap()
  };
  println!("The {}th number is {:?}.", player_input, f(player_input));
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

// // メールアドレスをチェックする関数
// // @スタートでない
// // スペースなし
// // @がひとつのみ
// // @のあとに.がある
// fn is_valid_email(email: String) -> bool {
//   vec![
//     !email.starts_with("@"),
//     email.split_whitespace().collect::<Vec<&str>>().len() == 1,
//     email.split("@").collect::<Vec<&str>>().len() == 2,
//     email
//       .split("@")
//       .collect::<Vec<&str>>()
//       .last()
//       .unwrap()
//       .split("")
//       .collect::<Vec<&str>>()
//       .contains(&"."),
//   ]
//   .iter()
//   .all(|&x| x == true)
// }

// フィボナッチ数列（動的計画法）
fn f(n: isize) -> isize {
  let mut now = 1;
  let mut p1 = 1;
  for _ in 2..n {
    let p2 = p1;
    p1 = now;
    now = p1 + p2;
  }
  now
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(f(1), 1);
    assert_eq!(f(2), 1);
    assert_eq!(f(3), 2);
    assert_eq!(f(4), 3);
    assert_eq!(f(5), 5);
    assert_eq!(f(6), 8);
    assert_eq!(f(7), 13);
    assert_eq!(f(8), 21);
    assert_eq!(f(9), 34);
  }
}
// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233,
