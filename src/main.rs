fn main() {
    println!("Hello, world!");
    for i in 1..10 {
        print!("fibonacci1 的 第{}位是{}  ", i, fibonacci1(i));
        println!("fibonacci2 的 第{}位是{}  ", i, fibonacci2(i));
    }

    println!("---------------摄氏度与华氏度转换--------------");

    let f_value: f32 = { 32f32 * 9f32 / 5f32 + 32f32 };
    println!("-------f32 :{}", 1f32 / 3f32);
    println!("32℃ = {f_value}℉");
    println!("32℃ = {}℉", convert_c_2_f(32f32));
    println!("32℉ = {}℃", convert_f_2_c(32f32));
}

///
/// 递归获取fibonacci数
fn fibonacci1(n: usize) -> usize {
    if n < 3 { 1 } else { fibonacci1(n - 1) + fibonacci1(n - 2) }
}

fn fibonacci2(n: usize) -> usize {
    let mut fibonacci_array: [usize; 10] = [0; 10];

    // 第一个数、第二个数 都是1
    fibonacci_array[1] = 1;
    fibonacci_array[2] = 1;

    for i in 3..=n {
        fibonacci_array[i] = fibonacci_array[i - 1] + fibonacci_array[i - 2];
    }

    fibonacci_array[n]
}

/// 摄氏温度c ，将其转化为华氏温度f ，转换公式为：f=c*9/5+32。
fn convert_c_2_f(c_value: f32) -> f32 {
    c_value * 9f32 / 5f32 + 32f32
}

fn convert_f_2_c(f_value: f32) -> f32 {
    (f_value - 32f32) * 5f32 / 9f32
}