extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  // println!("The secret number is: {}", secret_number); //秘密の数字は次の通り: {}

  loop {
    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    // 右のguessは入力された文字列の値．trim()は前後の空白と改行を削除する．
    // u32型は32ビットの非負整数．u32型は小さな非負整数のデフォルトの選択肢として丁度良い
    // parseの結果をmatchで受けて返す値を変える
    // 下のOrderingをmatchする流れと同じ
    // Okの場合はexpectが実行されて値をそのまま返す
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("You guessed: {}", guess);
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),  //小さすぎ！
      Ordering::Greater => println!("Too big!"), //大きすぎ！
      Ordering::Equal => {
        println!("You win!");
        break;
      } //やったね！
    }
  }
}
