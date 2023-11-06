use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::SystemTime;

fn main() {
    println!("Hello, world!");
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
