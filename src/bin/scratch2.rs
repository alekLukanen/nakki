#![allow(dead_code)]

use std::convert::From;

fn reverse(data: (i32, bool)) -> (bool, i32) {
    let (data_int, data_bool) = data;
    (data_bool, data_int)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

enum WebEvent {
    PageView,
    PageClick(i32),
    MousePosition { x: i32, y: i32 },
}

fn record_web_event(event: WebEvent) {
    match event {
        WebEvent::PageView => println!("page view"),
        WebEvent::PageClick(val) => println!("page click: {}", val),
        WebEvent::MousePosition { x, y } => println!("mouse position: ({}, {})", x, y),
    }
}

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

#[derive(Debug)]
enum Number {
    Zero,
    One,
    Two,
}

#[derive(Debug)]
struct NumberMod {
    x: i32,
}

impl From<i32> for NumberMod {
    fn from(value: i32) -> Self {
        NumberMod { x: value }
    }
}

fn main() {
    // tuples
    let data = (101, true);
    let reversed_data = reverse(data);

    println!("data: {:?}", data);
    println!("reversed data: {:?}", reversed_data);

    let matrix: Matrix = Matrix(1.0, 2.0, 3.0, 4.0);
    println!("matrix: {:?}", matrix);

    // arrays and slices
    let arr: [i32; 10] = [7; 10];
    for idx in 0..arr.len() {
        match arr.get(idx) {
            Some(arr_value) => println!("arr[{}]: {}", idx, arr_value),
            None => println!("unknown error"),
        }
    }

    // enums
    record_web_event(WebEvent::PageView);
    record_web_event(WebEvent::PageClick(42));
    record_web_event(WebEvent::MousePosition { x: 50, y: 60 });

    // enums :: use
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Stage::{Advanced, Beginner};
    // Automatically `use` each name inside `Role`.
    use crate::Role::*;

    // Equivalent to `Stage::Beginner`.
    let stage = Beginner;
    // Equivalent to `Role::Student`.
    let role = Student;

    match stage {
        // Note the lack of scoping because of the explicit `use` above.
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        // Note again the lack of scoping.
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }

    // enums :: indexing
    println!(
        "Number::Zero: {:?} as int is {}",
        Number::Zero,
        Number::Zero as i32,
    );
    println!(
        "Number::One: {:?} as int is {}",
        Number::One,
        Number::One as i32,
    );

    // type inference
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    vec.push(elem + 1);

    println!("vec: {:?}", vec);

    // from and into
    let x = 42;
    let sn = NumberMod::from(x);
    let x_mod: NumberMod = x.into();

    println!("x: {}, sn: {:?}, x_mod: {:?}", x, sn, x_mod);
}
