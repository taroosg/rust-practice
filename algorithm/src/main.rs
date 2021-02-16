fn main() {
  println!("Input One Line!");
  let player_input: u32 = {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_string().parse().unwrap()
  };
  println!(
    "The cube surface area is {}.",
    cube_surface_area(player_input)
  );
}

fn cube_surface_area(num: u32) -> u32 {
  num.pow(2) * 6
}
