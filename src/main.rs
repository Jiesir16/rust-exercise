fn main() {
    println!("Hello, world!");
    for i in 1..10 {
        print!("fibonacci1 的 第{}位是{}  ", i, fibonacci1(i));
        println!("fibonacci2 的 第{}位是{}  ", i, fibonacci2(i));
    }
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