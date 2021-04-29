fn main() {
  // println!("Input One Line!");
  // let player_input: usize = {
  //   let mut s = String::new();
  //   std::io::stdin().read_line(&mut s).unwrap();
  //   s.trim_end().to_string().parse().unwrap()
  // };
  // println!(
  //   "Input is {}, number is {:?}.",
  //   player_input,
  //   recursive_digits_added(player_input)
  // );
  dbg!(sum_of_all_primes(3));
}

// 素数のカウント
fn sum_of_all_primes(n: u32) -> u32 {
  // 何かが何かで割り切れるかどうかを判定する関数
  fn is_divisible_by_some(multiplicand: u32, multiplier: u32) -> bool {
    match multiplicand {
      0 | 1 => false,
      _ => match multiplier {
        1 => true,
        _ => match multiplicand % multiplier {
          0 => false,
          _ => is_divisible_by_some(multiplicand, multiplier - 1),
        },
      },
    }
  }
  // numberが素数かどうかを判定する関数
  fn is_prime(number: u32) -> bool {
    is_divisible_by_some(number, (number as f32).sqrt().floor() as u32)
  }
  // 配列内の素数のみを残す関数
  fn create_prime_array(array: Vec<u32>) -> Vec<u32> {
    array.iter().filter(|&x| is_prime(*x)).map(|&x| x).collect()
  }
  // 配列内の数を合計する関数
  fn get_sum_of_array(array: Vec<u32>) -> u32 {
    array.iter().sum()
  }
  // 0からnまでの配列を作り，素数のみの配列にし，合計値を算出
  get_sum_of_array(create_prime_array((0..n + 1).collect()))
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(sum_of_all_primes(1), 0);
    assert_eq!(sum_of_all_primes(2), 2);
    assert_eq!(sum_of_all_primes(3), 5);
    assert_eq!(sum_of_all_primes(100), 1060);
    assert_eq!(sum_of_all_primes(1000), 76127);
  }
}

// 10進数→2進数の変換
// fn decimal_to_binary(dec_number: u32) -> String {
//   fn get_quotient_divided_by_2(number: u32) -> u32 {
//     number / 2
//   }
//   fn get_surplus_divided_by_2(number: u32) -> u32 {
//     number % 2
//   }
//   fn unshift_number_to_array(number: u32, array: Vec<u32>) -> Vec<u32> {
//     [number].iter().chain(array.iter()).map(|&x| x).collect()
//   }
//   fn create_binary_array(number: u32, array: Vec<u32>) -> Vec<u32> {
//     match get_quotient_divided_by_2(number) {
//       0 => unshift_number_to_array(get_surplus_divided_by_2(number), array),
//       _ => create_binary_array(
//         get_quotient_divided_by_2(number),
//         unshift_number_to_array(get_surplus_divided_by_2(number), array),
//       ),
//     }
//   }
//   fn join_array(array: Vec<u32>) -> String {
//     array.iter().map(|&x| x.to_string()).collect()
//   }
//   join_array(create_binary_array(dec_number, vec![]))
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   #[test]
//   fn it_works() {
//     assert_eq!(decimal_to_binary(60), "111100");
//     assert_eq!(decimal_to_binary(26), "11010");
//     assert_eq!(decimal_to_binary(35), "100011");
//     assert_eq!(decimal_to_binary(100), "1100100");
//     assert_eq!(decimal_to_binary(505), "111111001");
//   }
// }

// 数字の分割
// fn recursive_digits_added(n: usize) -> usize {
//   fn get_splitted_array_from_number(number: usize) -> Vec<usize> {
//     number
//       .to_string()
//       .chars()
//       .map(|c| c.to_digit(10).unwrap() as usize)
//       .collect()
//   }
//   fn get_sum_from_splitted_array(number_array: &Vec<usize>) -> usize {
//     number_array.iter().sum()
//   }
//   fn unshift_number_to_array(number: usize, array: Vec<usize>) -> Vec<usize> {
//     [number].iter().chain(&array).map(|&x| x).collect()
//   }
//   fn generate_result_array(number: usize, result_array: Vec<usize>) -> Vec<usize> {
//     let splitted_array = get_splitted_array_from_number(number);
//     let sum = get_sum_from_splitted_array(&splitted_array);
//     let new_array = unshift_number_to_array(sum, result_array);
//     match splitted_array.len() {
//       1 => new_array,
//       _ => generate_result_array(sum, new_array),
//     }
//   }
//   let result_array: Vec<usize> = generate_result_array(n, vec![]);
//   match result_array.len() {
//     1 => result_array.iter().sum(),
//     _ => result_array.iter().skip(1).sum(),
//   }
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   #[test]
//   fn it_works() {
//     assert_eq!(recursive_digits_added(5), 5);
//     assert_eq!(recursive_digits_added(8), 8);
//     assert_eq!(recursive_digits_added(12), 3);
//     assert_eq!(recursive_digits_added(98), 25);
//     assert_eq!(recursive_digits_added(3528), 27);
//     assert_eq!(recursive_digits_added(99999999999884), 132);
//     assert_eq!(recursive_digits_added(5462), 25);
//     assert_eq!(recursive_digits_added(45622943), 43);
//     assert_eq!(recursive_digits_added(9514599), 48);
//   }
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

// // きっぷの価格から購入人数を求める関数
// fn weekly_7days_sales(ticket_price: i32) -> i32 {
//   match (250 - ticket_price) * 700 + 150000 {
//     n if n < 0 => 0,
//     n => n,
//   }
// }

// // 豆腐の一辺から表面積を求める関数
// fn cube_surface_area(num: u32) -> u32 {
//   num.pow(2) * 6
// }
