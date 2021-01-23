#[derive(Debug)]

struct Rectangle {
  width: u32,
  height: u32,
}

// 構造体`Rectangle`に対する`area`メソッドを定義
impl Rectangle {
  // `self`は構造体`Rectangle`のこと
  fn area(&self) -> u32 {
    self.width * self.height
  }

  // 四角が他の四角の中に入るかどうか
  // 別の四角は`Rectangle`の参照を借りる
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  // mutを入れると再代入できる
  // let mut x = 5;
  // println!("The value of x is: {}", x); // xの値は{}です
  // x = 6;
  // println!("The value of x is: {}", x);

  // let x = 5;
  // // これは再代入ではなくシャドーイングと呼ぶ（更新に近い）
  // let x = x + 1;
  // let x = x * 2;

  // println!("The value of x is: {}", x);

  // 型が異なるのでmutを使った再代入はできない
  // let spaces = "   ";
  // let spaces = spaces.len();
  // println!("The number of space is: {}", spaces);

  // 静的型付言語なので，コンパイル時に型が判明していないといけない
  // : u32が必要
  // let guess: u32 = "42".parse().expect("Not a number!"); // 数字ではありません！
  // println!("The number is: {}", guess);

  // // 足し算
  // let sum: i32 = 5 + 10;

  // // 引き算
  // let difference: f64 = 95.5 - 4.3;

  // // 掛け算
  // let product: i32 = 4 * 30;

  // // 割り算
  // let quotient: f64 = 56.7 / 32.2;

  // // 余り
  // let remainder: i32 = 43 % 5;

  // println!("The sum is: {}", sum);
  // println!("The difference is: {}", difference);
  // println!("The product is: {}", product);
  // println!("The quotient is: {}", quotient);
  // println!("The remainder is: {}", remainder);

  // タプル
  // let tup: (i32, f64, u8) = (500, 6.4, 1);
  // let (x, y, z) = tup;

  // 分割代入とインデックスでとれる
  // println!("The tup[0] is: {}", x);
  // println!("The tup[1] is: {}", tup.2);

  // // 配列は固定長
  // let months = [
  //   "January",
  //   "February",
  //   "March",
  //   "April",
  //   "May",
  //   "June",
  //   "July",
  //   "August",
  //   "September",
  //   "October",
  //   "November",
  //   "December",
  // ];
  // // 繰り返し
  // for elm in &months {
  //   println!("The month is: {}", elm);
  // }

  // // 関数呼出し
  // let hoge = another_function(10);
  // println!("number is {}", hoge);
  // let number = 6;

  // let result = if number % 4 == 0 {
  //   // 数値は4で割り切れます
  //   "number is divisible by 4"
  // } else if number % 3 == 0 {
  //   // 数値は3で割り切れます
  //   "number is divisible by 3"
  // } else if number % 2 == 0 {
  //   // 数値は2で割り切れます
  //   "number is divisible by 2"
  // } else {
  //   // 数値は4、3、2で割り切れません
  //   "number is not divisible by 4, 3, or 2"
  // };
  // println!("result is {}", result);

  // // `&`をつけて変数を参照
  // let s1 = String::from("☕🍻🎓");
  // let len = calculate_length(&s1);
  // println!("The length of '{}' is {}.", s1, len);

  // // `s`はミュータブルな変数
  // let mut s = String::from("hello");

  // change(&mut s);
  // println!("New string is {}.", s);

  // let mut s = String::from("hello world");
  // let word = first_word(&s); // wordの中身は、値5になる
  // println!("{}", word);
  // s.clear(); // Stringを空にする。つまり、""と等しくする

  // // 配列のスライス
  // let a = [1, 2, 3, 4, 5];
  // let slice = &a[1..3];
  // println!("{:?}", slice);

  // // 構造体の定義
  // struct Color(i32, i32, i32);
  // struct Point(i32, i32, i32);

  // let black = Color(0, 0, 0);
  // let origin = Point(0, 0, 0);

  // println!("{:?}", origin.0)

  // // 構造体を使ったプログラム例

  // // 高さと幅を別々の変数で定義
  // // let width1 = 30;
  // // let height1 = 50;
  // // タプルを使う
  // // let rect1 = (30, 50);
  // // 構造体を使う
  // let rect1 = Rectangle {
  //   width: 30,
  //   height: 50,
  // };
  // let rect2 = Rectangle {
  //   width: 10,
  //   height: 40,
  // };
  // let rect3 = Rectangle {
  //   width: 60,
  //   height: 45,
  // };

  // println!("{:#?}", rect2);

  // println!(
  //   // 長方形の面積は、{}平方ピクセルです
  //   "The area of the rectangle is {} square pixels.",
  //   // rect2がselfになる
  //   rect2.area()
  // );
  // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  // println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

  // enum型の`IpAddrKind`を定義
  enum IpAddrKind {
    V4,
    V6,
  }

  // 構造体`IpAddr`を定義
  // struct IpAddr {
  //   kind: IpAddrKind,
  //   address: String,
  // }

  //   // まとめられる
  //   enum IpAddr {
  //     V4(u8, u8, u8, u8),
  //     V6(String),
  // }

  // let home = IpAddr::V4(127, 0, 0, 1);

  // let loopback = IpAddr::V6(String::from("::1"));

  // // `home`は`IpAddr`の構造体
  // let home = IpAddr {
  //   // `kind`は`IpAddrKind`型の`V4`
  //   kind: IpAddrKind::V4,
  //   // `address`は文字列
  //   address: String::from("127.0.0.1"),
  // };

  // let loopback = IpAddr {
  //   kind: IpAddrKind::V6,
  //   address: String::from("::1"),
  // };

  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
  }

  fn value_in_cents(coin: Coin) -> u32 {
    match coin {
      Coin::Penny => {
        println!("Lucky penny!");
        1
      }
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
    }
  }
  let result = value_in_cents(Coin::Nickel);
  println!("{:?}", result);
}

// // 外部関数定義
// fn another_function(x: i32) -> i32 {
//   x * 2
// }

// // 文字列を入力して長さを返す関数
// // `&String`が文字列の参照を表している
// fn calculate_length(s: &String) -> usize {
//   // `s`のスコープはこの中だけ
//   s.len()
// }

// // 入力文字列になにか追加する関数
// fn change(some_string: &mut String) {
//   some_string.push_str(", world");
// }

// fn first_word(s: &String) -> &str {
//   // Stringをバイト配列に変換する
//   let bytes = s.as_bytes();
//   // バイト配列を文字列に変換
//   // println!("bytes = {}", String::from_utf8(bytes.to_vec()).unwrap());

//   // スペースがあったら何番目かを返す
//   for (i, &item) in bytes.iter().enumerate() {
//     if item == b' ' {
//       return &s[0..i];
//     }
//   }
//   // スペースがなければ文字列の長さを返す
//   &s[..]
// }

// fn area(rectangle: &Rectangle) -> u32 {
//   rectangle.width * rectangle.height
// }
