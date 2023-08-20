fn main() {
  println!("Hello, world!");
  alura_pattern_matching();
}

fn alura_pattern_matching() {
  println!("---> Alura Pattern Matching");
  for x in 1..=20 {
    println!("{}: {}", x, match x {
      1 => "litte",
      2 | 3 => "something",
      4..=10 => "a lot",
      _ if x % 2 == 0 => "good",
      _ => "very much"
    })
  }

  let point = (0, 1);
  println!("{}",
    match point {
      (0,0) => "origin",
      (0,_) => "axis x",
      (_,0) => "axis y",
      (_,_) => "alone"
    }
  )
}
