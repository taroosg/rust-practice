fn main() {
  println!("Input One Line!");
  let player_input: usize = {
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
// fn f(n: isize) -> isize {
//   let mut now = 1;
//   let mut p1 = 1;
//   for _ in 2..n {
//     let p2 = p1;
//     p1 = now;
//     now = p1 + p2;
//   }
//   now
// }

// 数字の分割
fn recursive_digits_added(n: usize) -> usize {
  fn get_splitted_array_from_number(number: usize) -> Vec<usize> {
    number
      .to_string()
      .chars()
      .map(|c| c.to_digit(10).unwrap() as usize)
      .collect::<Vec<usize>>()
  }
  fn get_sum_from_splitted_array(number_array: &Vec<usize>) -> usize {
    number_array.iter().sum()
  }
  fn unshift_number_to_array(number: usize, array: Vec<usize>) -> Vec<usize> {
    [number].iter().chain(&array).map(|&x| x).collect()
  }
  fn generate_result_array(number: usize, result_array: Vec<usize>) -> Vec<usize> {
    let splitted_array = get_splitted_array_from_number(number);
    let sum = get_sum_from_splitted_array(&splitted_array);
    let new_array = unshift_number_to_array(sum, result_array);
    match splitted_array.len() {
      1 => return new_array,
      _ => return generate_result_array(sum, new_array),
    }
  }
  let result_array: Vec<usize> = generate_result_array(n, vec![]);
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
    assert_eq!(recursive_digits_added(99999999999884), 132);
    assert_eq!(recursive_digits_added(5462), 25);
    assert_eq!(recursive_digits_added(45622943), 43);
    assert_eq!(recursive_digits_added(9514599), 48);
  }
}
// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233,
