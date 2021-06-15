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
  println!("hello");
}

// 1からNまでの数の各桁の和がA以上B以下のものの総和を求める
fn some_sums(n: usize, a: usize, b: usize) -> usize {
  // 1からnまでの数値が入った配列を作成する関数
  fn create_1_to_n_array(n: usize) -> Vec<usize> {
    vec![0; n]
      .iter()
      .enumerate()
      .map(|(i, _x)| i + 1)
      .collect::<Vec<usize>>()
  }
  // 文字列の配列を数値の配列にする関数
  fn str_array_to_number_array(array: Vec<String>) -> Vec<usize> {
    array.iter().map(|x| x.parse::<usize>().unwrap()).collect()
  }
  // 文字列を文字列の配列にする関数
  fn str_to_vec(s: String) -> Vec<String> {
    s.split("")
      .filter(|&x| x != "")
      .map(|x| x.to_string())
      .collect::<Vec<String>>()
  }
  // 数値の配列の和を出力する関数
  fn get_sum(array: Vec<usize>) -> usize {
    array.iter().sum()
  }
  // a以上b以下ならtrueにする関数
  fn is_a_to_b(n: usize, a: usize, b: usize) -> bool {
    a <= n && n <= b
  }
  // 文字列の配列の各要素の桁の和がa以上b以下のものだけ残す関数
  fn filter_a_to_b(array: Vec<usize>, a: usize, b: usize) -> usize {
    array
      .iter()
      .filter(|&x| {
        is_a_to_b(
          get_sum(str_array_to_number_array(str_to_vec(x.to_string()))),
          a,
          b,
        )
      })
      .sum::<usize>()
  }
  filter_a_to_b(create_1_to_n_array(n), a, b)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    main();
    assert_eq!(some_sums(20, 2, 5), 84);
  }
}

// // 2で割れる回数を出力
// fn how_many_devides(s: &str) -> usize {
//   // 文字列を配列にする関数
//   fn string_to_usize_vec(s: &str) -> Vec<usize> {
//     s.split_ascii_whitespace()
//       .collect::<Vec<_>>()
//       .iter()
//       .map(|&x| x.parse::<usize>().unwrap())
//       .collect()
//   }
//   // 配列の要素が全部2で割れるかどうか確認する関数
//   fn can_all_devide_2(array: &Vec<usize>) -> bool {
//     array.iter().all(|x| x % 2 == 0)
//   }
//   // 配列の全要素を2で割った配列をつくる関数
//   fn devide_2(array: Vec<usize>) -> Vec<usize> {
//     array.iter().map(|x| x / 2).collect()
//   }
//   // 2で割り続けて回数を出力する関数
//   fn super_devide(array: Vec<usize>, count: usize) -> usize {
//     match can_all_devide_2(&array) {
//       true => super_devide(devide_2(array), count + 1),
//       false => count,
//     }
//   }
//   super_devide(string_to_usize_vec(s), 0)
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   #[test]
//   fn it_works() {
//     main();
//     assert_eq!(how_many_devides("12 24 16"), 2);
//     assert_eq!(how_many_devides("12 24 35"), 0);
//   }
// }

// // 01文字列の中の1の個数
// fn get_number_of_1(s: &str) -> usize {
//   s.split("")
//     .collect::<Vec<_>>()
//     .iter()
//     .map(|&x| x.to_string())
//     .filter(|x| x == "1")
//     .collect::<Vec<_>>()
//     .len()
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   #[test]
//   fn it_works() {
//     main();
//     assert_eq!(get_number_of_1("111001"), 4);
//     assert_eq!(get_number_of_1("0000"), 0);
//   }
// }

// // 積の偶数奇数判定
// fn judge_odd_even(a: usize, b: usize) -> String {
//   vec!["even", "odd"][(a * b) % 2].to_string()
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   #[test]
//   fn it_works() {
//     main();
//     assert_eq!(judge_odd_even(3, 4), "even");
//     assert_eq!(judge_odd_even(1, 21), "odd");
//   }
// }

// // 従業員の解雇
// fn fire_employees(employees: Vec<&str>, unemployed: Vec<&str>) -> Vec<String> {
//   employees
//     .iter()
//     .filter(|&x| !unemployed.contains(x))
//     .map(|&x| x.to_string())
//     .collect()
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   #[test]
//   fn it_works() {
//     let empty: Vec<String> = vec![];
//     assert_eq!(
//       fire_employees(
//         vec!["Steve", "David", "Mike", "Donald", "Lake", "Julian"],
//         vec!["Donald", "Lake"]
//       ),
//       vec!["Steve", "David", "Mike", "Julian"]
//     );
//     assert_eq!(
//       fire_employees(vec!["Donald", "Lake"], vec!["Donald", "Lake"]),
//       empty
//     );
//     assert_eq!(
//       fire_employees(
//         vec!["Steve", "David", "Mike", "Donald", "Lake", "Julian"],
//         vec![]
//       ),
//       vec!["Steve", "David", "Mike", "Donald", "Lake", "Julian"]
//     );
//     assert_eq!(
//       fire_employees(
//         vec!["Mike", "Steve", "David", "Mike", "Donald", "Lake", "Julian"],
//         vec!["Mike"]
//       ),
//       vec!["Steve", "David", "Donald", "Lake", "Julian"]
//     );
//   }
// }

// // 素数のカウント
// fn sum_of_all_primes(n: u32) -> u32 {
//   // 何かが何かで割り切れるかどうかを判定する関数
//   fn is_divisible_by_some(multiplicand: u32, multiplier: u32) -> bool {
//     match multiplicand {
//       0 | 1 => false,
//       _ => match multiplier {
//         1 => true,
//         _ => match multiplicand % multiplier {
//           0 => false,
//           _ => is_divisible_by_some(multiplicand, multiplier - 1),
//         },
//       },
//     }
//   }
//   // numberが素数かどうかを判定する関数
//   fn is_prime(number: u32) -> bool {
//     is_divisible_by_some(number, (number as f32).sqrt().floor() as u32)
//   }
//   // 配列内の素数のみを残す関数
//   fn create_prime_array(array: Vec<u32>) -> Vec<u32> {
//     array.iter().filter(|&x| is_prime(*x)).map(|&x| x).collect()
//   }
//   // 配列内の数を合計する関数
//   fn get_sum_of_array(array: Vec<u32>) -> u32 {
//     array.iter().sum()
//   }
//   // 0からnまでの配列を作り，素数のみの配列にし，合計値を算出
//   get_sum_of_array(create_prime_array((0..n + 1).collect()))
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   #[test]
//   fn it_works() {
//     main();
//     assert_eq!(sum_of_all_primes(1), 0);
//     assert_eq!(sum_of_all_primes(2), 2);
//     assert_eq!(sum_of_all_primes(3), 5);
//     assert_eq!(sum_of_all_primes(100), 1060);
//     assert_eq!(sum_of_all_primes(1000), 76127);
//   }
// }

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

// // フィボナッチ数列（動的計画法）
// fn f(n: usize) -> usize {
//   let mut now = 1;
//   let mut p1 = 1;
//   for _ in 2..n {
//     let p2 = p1;
//     p1 = now;
//     now = p1 + p2;
//   }
//   now
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   #[test]
//   fn it_works() {
//     main();
//     assert_eq!(f(1), 1);
//     assert_eq!(f(5), 5);
//     assert_eq!(f(10), 55);
//     assert_eq!(f(90), 2880067194370816120);
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
