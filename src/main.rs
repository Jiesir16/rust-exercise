use std::fs::File;
use std::io::{Read, stdin};
use std::path::Path;
use std::time::SystemTime;

use rand::random;

use crate::Message::Quit;

fn main() {
    println!("Hello, world!");
    Message::Move(100, 20).call();
    Message::ChangeColor(143, 254, 255).call();
    Message::Write(String::from("你好啊")).call();
    Quit.call();

    let x = 5;
    let y = get_i32(random());

    let z = match y {
        Some(i) => i + x,
        _ => {
            println!("the y is other!");
            x
        }
    };
    println!("--------z value : {z}");

    let z = if let Some(i) = y { i + x } else { x };
    println!("--------z value : {z}");
    let mut arg = String::new();
    stdin().read_line(&mut arg).expect("获取用户输入失败!");
    match arg.trim() {
        "md5" => cal_md5(),
        _ => println!("错误值")
    }
}

fn cal_md5() {
    let start_time = SystemTime::now();
    let file_path = Path::new("/Users/jiesir/JSpace/java后端开发必备手册.7z");
    let mut file = File::open(file_path).expect("cuowule");
    let mut data = vec![];
    file.read_to_end(&mut data).expect("cuowule");
    let md5_hash = md5::compute(&data);
    let md5_hash = format!("{:2x}", &md5_hash);
    let file_path = file_path.to_str().expect("获取文件名称错误");
    let end_time = SystemTime::now();
    println!("{file_path} md5值为:{md5_hash},用时{}ms", (end_time.duration_since(start_time).expect("计算时间差值错误").as_millis()));
}

enum Message {
    Quit,
    Move(i32, i32),
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Quit => println!("this is Quit."),
            Message::Move(x, y) => println!("this is move.point x :{x},y:{y}"),
            Message::Write(s) => println!("this is Write. s :{s}"),
            Message::ChangeColor(i1, i2, i3) => println!("this is change color. #{:2X}{:2X}{:2X}", i1, i2, i3)
        }
    }
}

fn get_i32(i: i8) -> Option<i32> {
    match i {
        1 => None,
        2 => None,
        3 => None,
        _ => Some(i as i32)
    }
}