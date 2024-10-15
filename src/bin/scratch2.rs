#![allow(dead_code)]

use core::fmt;
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

struct Circle {
    radius: f32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle radious is {}", self.radius)
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

    // string conversions
    let circle = Circle { radius: 5.0 };
    println!("Circle: {}", circle);

    let int_val1: i32 = match "00008".parse() {
        Ok(val) => val,
        Err(e) => {
            println!("error: {}", e);
            return;
        }
    };
    let int_val2 = match "10".parse::<i32>() {
        Ok(val) => val,
        Err(e) => {
            println!("error: {}", e);
            return;
        }
    };

    println!("int_val1: {}, int_val2: {}", int_val1, int_val2);

    // loops
    let mut outer_loop_val: i32 = 0;
    let mut inner_loop_val: i32 = 0;

    'outer_loop: loop {
        println!("outer_loop index {}", outer_loop_val);
        outer_loop_val += 1;
        if outer_loop_val >= 5 {
            break 'outer_loop;
        }

        loop {
            println!("inner_loop index {}", inner_loop_val);
            inner_loop_val += 1;

            if inner_loop_val % 2 == 0 {
                continue 'outer_loop;
            } else {
                break;
            }
        }

        println!("after inner loop");
    }

    let mut looper1 = {
        let mut count = 1;
        loop {
            count *= 10;
            if count >= 44 {
                break;
            }
        }
        count
    };
    println!("looper1: {}", looper1);

    let looper2 = loop {
        looper1 += 1;
        if looper1 >= 125 {
            break looper1;
        }
    };
    println!("looper2: {}", looper2);
}
