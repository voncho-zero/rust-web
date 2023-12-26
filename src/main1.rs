use std::{io, thread};
use rand::Rng;
use std::cmp::Ordering;
use restaurant::eat_at_restaurant;

use std::collections::HashMap;
use std::sync::mpsc;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}
fn main() {
    println!("Hello, world!");
    println!("世界你好");
    let _a = "hide".to_string();
    let _a = "hide".to_string();
    let mut str = String::from("hi");
    say_hi(&mut str);
    println!("{}", str);
    say_h(_a);
    // println!("{}", _a);
    // guess_number();
    let rec = Rectangle {
        width: 30,
        height: 40
    };
    let square = Rectangle::square(10);
    println!("{}", rec.area());

    println!("{}", square.area());

    let dice_roll = 9;
    let result = match dice_roll {
        3 => 3,
        7 => 3,
        _other => 0,
    };
    eat_at_restaurant();
    let v = vec![1, 2, 3];
    let mut a = v[1];
    println!("{}", a);
    a = 4;
    println!("{}", v[1]);
    let mut v = Vec::new();
    v.push(4);
    v.push(5);
    if let Some(i) = v.get(1) {
        println!("{}", i)
    };
    let sum = v.iter().sum::<i32>();
    println!("sum:{sum}");

    let mut map = HashMap::new();
    map.insert(String::from("a"), 10);
    for (k, v) in &map {
        println!("{k}:{v}")
    }
    println!("{:?}", map);
    let a : i32 = 9;
    let b = &a;
    println!("{}", b);
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);


    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("{}", received);

    let arr = [1, 2, 3];
    if let [a@ ..] = arr {

    }
    
}

fn say_hi(msg: &mut String) {
    msg.push_str("a");
    println!("{}", msg)
}

fn say_h(msg: String) {
    println!("{}", msg)
}

fn guess_number() {
    println!("猜数字游戏");


    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("请输入猜测的数字");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("读取数字失败");
        println!("输入的数字是: {guess}");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            },
        }
    }


}
