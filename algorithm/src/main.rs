fn main() {
  println!("Input One Line!");
  let player_input: u32 = {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_string().parse().unwrap()
  };
  println!(
    "Input is {}, number is {:?}.",
    player_input,
    recursive_digits_added(player_input)
  );
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

// 数字の分割
fn recursive_digits_added(n: u32) -> u32 {
  let mut result_array = vec![];
  fn a(num: u32, result_array: &mut Vec<u32>) {
    let num_array: Vec<u32> = num
      .to_string()
      .chars()
      .map(|c| c.to_digit(10).unwrap())
      .collect();
    let array_sum: u32 = num_array.iter().sum();
    result_array.insert(0, array_sum);
    match num_array.len() {
      1 => return,
      _ => a(array_sum, result_array),
    }
  }
  a(n, &mut result_array);
  match result_array.len() {
    1 => result_array.iter().sum(),
    _ => result_array.iter().skip(1).sum(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(recursive_digits_added(5), 5);
    assert_eq!(recursive_digits_added(8), 8);
    assert_eq!(recursive_digits_added(12), 3);
    assert_eq!(recursive_digits_added(98), 25);
    assert_eq!(recursive_digits_added(3528), 27);
    // assert_eq!(recursive_digits_added(99999999999884), 132);
    assert_eq!(recursive_digits_added(5462), 25);
    assert_eq!(recursive_digits_added(45622943), 43);
    assert_eq!(recursive_digits_added(9514599), 48);
  }
}
// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233,
