use std::io;

// Constants
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
  // compound_types();
  // array_type();
  // invalid_array_element_access();
  alura_conditions();
  // alura_repetitions();
  // rust_variables();
  // rust_functions();
  // rust_control_flow();
}

fn rust_control_flow() {
  println!("---> if Expressions");
  let a = 6;
  if a % 4 == 0 {
      println!("a is divisible by 4");
  } else if a % 3 == 0 {
      println!("a is divisible by 3");
  } else if a % 2 == 0 {
      println!("a is divisible by 2");
  } else {
      println!("a is not divisible by 4, 3, or 2");
  }

  println!("---> Using if in a let Statement");
  let condition = true;
  let x: i32 = if condition { 5 } else { 6 };
  println!("The value of x is: {x}");

  println!("---> Repetition with Loops");
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 { break counter * 2; }
  };
  println!("The result is {result}");

  println!("---> Loop Labels to Disambiguate Between Multiple Loops");
  let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 { break; }
      if count == 2 { break 'counting_up; }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {count}");

  println!("---> Conditional Loops with while");
  let mut number = 3;
  while number != 0 {
    println!("Using while, {number}!");
    number -= 1;
  }
  for number in (1..4).rev() {
    println!("Using for, {number}!");
  }
  for number in (1..=3).rev() {
    println!("Using for*, {number}!");
  }

  println!("---> Looping Through a Collection with for");
  let a = [10, 20, 30, 40, 50];
  let mut index = 0;
  while index < 5 {
    println!("Using while, the value is: {}", a[index]);
    index += 1;
  }
  for element in a {
    println!("Using for, the value is: {element}");
  }


}

fn alura_repetitions() {
  let multiplier: u8 = 5;

  println!("Multiplication table using WHILE");
  let mut counter: u8 = 0;
  while counter < 10 {
    counter += 1;

    if counter == 5 { continue; }

    println!("{} * {} = {}", multiplier, counter, multiplier * counter);
  }

  println!("Multiplication table using LOOP");
  counter = 0;
  loop {
    counter += 1;
    println!("{} * {} = {}", multiplier, counter, multiplier * counter);

    if counter == 10 { break; }
  }
}

fn rust_functions() {
  another_function(32);
  print_labeled_measurement(5, 'm');
  statements_and_expressions();
  with_return_values();
}

fn another_function(x: i32) {
  println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}

fn statements_and_expressions() {
  println!("---> Statements and Expressions");
  // statement
  let a = 6;
  println!("The value of a is: {a}");
  // expression
  let y = {
    let x = 3;
    x + 1
  };
  println!("The value of y is: {y}");
}

fn with_return_values() {
  println!("---> Functions with Return Values");
  let x = plus_one(5);
  println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 { x + 1 }

fn alura_conditions() {
  println!("---> Alura Conditions");
  let age = 17;
  let responsible_authorization = true;

  println!("How old are you ?");
  let mut age: String = String::new();
  io::stdin()
      .read_line(&mut age)
      .expect("Failed to read line");
  let age: usize = age
      .trim()
      .parse()
      .expect("Age entered was not a number");
  let is_adult = age >= 18;

  if is_adult {
    println!("Authorized !");
  } else if age > 16 && responsible_authorization {
    println!("Authorized with conditions !")
  } else {
    println!("Unauthorized !")
  }

  let condition = if is_adult { "maior" } else { "menor" };
  print!("É {} de idade !", condition);

  println!("---> Alura Conditions");
  let language = "PHP";
  let purpose = match language {
    "PHP" => "Web",
    "Kotlin" => "Android",
    "Python" => "Data Science",
    _ => "Undefined"
  };
  println!("The purpose of {language} is {purpose}.");
}

fn rust_variables() {
  let mut x = 5;
  println!("The value of x is: {x}");
  x = 6;
  println!("The value of x is: {x}");

  // Shadowing
  let y = 5;
  let y = y + 1;
  {
    let y = y * 2;
    println!("The value of y in the inner scope is: {y}");
  }
  println!("The value of y is: {y}");

  let spaces = "   ";
  let spaces = spaces.len();

  let mut spaces = "   ";
  // spaces = spaces.len();

  // Data Types
  // ---> Floating-Point Types
  let x = 2.2; // f64
  let y: f32 = 3.0; // f32
  println!("x = {x}; y = {y}");

  // ---> Numeric Operations
  // addition
  let sum = 5 + 10;
  // subtraction
  let difference = 95.5 - 4.3;
  // multiplication
  let product = 4 * 30;
  // division
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3; // Results in -1
  // remainder
  let remainder = 43 % 5;

  // ---> The Boolean Type
  let t = true;
  let f: bool = false; // with explicit type annotation
  
  // ---> The Character Type
  let c = 'z';
  let z: char = 'ℤ'; // with explicit type annotation
  let heart_eyed_cat = '😻';
  // Note that we specify char literals with single quotes,
  // as opposed to string literals, which use double quotes
}

fn compound_types() {
  // Compound Types
  println!("---> The Tuple Type");
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;

  println!("The value of x is: {x}; access directly: {}", tup.0);
  println!("The value of y is: {y}; access directly: {}", tup.1);
  println!("The value of z is: {z}; access directly: {}", tup.2);
}

fn array_type() {
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  let first = a[0];
  let second = a[1];
  println!("Values of array: first = {first} and second = {second}");

  let b = [3; 5];
}

fn invalid_array_element_access() {
  let a = [1, 2, 3, 4, 5];

  println!("Please enter an array index.");

  let mut index: String = String::new();

  io::stdin()
      .read_line(&mut index)
      .expect("Failed to read line");

  let index: usize = index
      .trim()
      .parse()
      .expect("Index entered was not a number");

  let element = a[index];

  println!("The value of the element at index {index} is: {element}");
}
