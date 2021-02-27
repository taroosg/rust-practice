fn main() {
  println!("Input One Line!");
  let player_input: i32 = {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_string().parse().unwrap()
  };
  println!("{} peaple buy tiskets.", weekly_7days_sales(player_input));
}

fn cube_surface_area(num: u32) -> u32 {
  num.pow(2) * 6
}

fn weekly_7days_sales(ticket_price: i32) -> i32 {
  match (250 - ticket_price) * 700 + 150000 {
    n if n < 0 => 0,
    n => n,
  }
}
