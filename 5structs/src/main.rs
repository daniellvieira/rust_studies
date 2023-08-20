struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}
impl Rectangle {
  fn width(&self) -> bool {
    self.width > 0
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }
  
  fn double(&mut self) {
    self.width *= 2;
    self.height *= 2;
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  // Associated Functions
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

fn main() {
  defining_and_instantiating_structs();
  tuple_structs();
  an_example_program();
  method_syntax();
}

fn defining_and_instantiating_structs() {
  println!("---> Defining and Instantiating Structs");
  let mut user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
  };
  let user2 = User {
    email: String::from("another@example.com"),
    ..user1
  };

  user1.email = String::from("changed@example.com");
  // println!("email: {} - username: {}", user1.email, user1.username);
}

fn build_user(email: String, username: String) -> User {
  User {
    active: true,
    username,
    email,
    sign_in_count: 1,
  }
}

fn tuple_structs() {
  println!("---> Tuple Structs");
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}

fn an_example_program() {
  println!("---> An Example Program Using Structs");
  // Version 1
  // let width1 = 30;
  // let height1 = 50;
  // println!(
  //     "The area of the rectangle is {} square pixels.",
  //     area(width1, height1)
  // );

  // Version 2
  // let rect1 = (30, 50);
  // println!(
  //   "The area of the rectangle is {} square pixels.",
  //   area(rect1)
  // );

  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  let scale = 2;
  let rect2 = Rectangle {
    width: dbg!(30 * scale),
    height: 50,
  };
  println!("rect1 is {:#?}", rect1);
  dbg!(&rect2);
  println!(
    "The area of the rectangle is {} square pixels.",
    area(&rect1)
  );
}

// Version 1
// fn area(width: u32, height: u32) -> u32 {
//   width * height
// }

// Version 2
// fn area(dimensions: (u32, u32)) -> u32 {
//   dimensions.0 * dimensions.1
// }

fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

fn method_syntax() {
  println!("---> Method Syntax");
  let mut rect1 = Rectangle {
    width: 20,
    height: 10,
  };
  rect1.double();
  if rect1.width() {
    println!("The rectangle has a nonzero width; it is {}.", rect1.width);
  }

  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );

  methods_with_mode_parameters();
  associated_functions();
}

fn methods_with_mode_parameters() {
  println!("-|| Methods with More Parameters");
  let mut rect1 = Rectangle {
    width: 1,
    height: 1,
  };
  let rect2 = Rectangle {
    width: 60,
    height: 45,
  };
  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  while !rect1.can_hold(&rect2) {
    rect1.double();
    println!(
      "rect1 is doubled and now has: width = {} and height = {}",
      rect1.width,
      rect1.height
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  };
}

fn associated_functions() {
  println!("-|| Associated Functions");
  let sq = Rectangle::square(3);
  dbg!(sq);
}