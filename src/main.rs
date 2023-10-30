fn main() {
    println!("-------------------------所有权 ownership-------------------------");

    let str1 = String::from("hello");

    // 非基础数据类型，值变量变更绑定，会将会把所有权给到新的变量
    let str2 = str1;
    // 此时str1已经无效，不能再使用,会报错
    // println!("str2 : {str2},str1: {}", str1);
    println!("str2 : {str2}");

    let mut str3 = give_ownership();
    println!("str3: {}", str3);
    str3.push_str(",world!");
    println!("str3: {}", str3);

    let (_str_sss, len) = calculate_length(str3);
    println!("str3: {},len: {}", _str_sss, len);

    println!("-------------------------借用和引用 borrow&reference-------------------------");
    println!("在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。");
    println!("引用必须总是有效的。");
    let mut str4 = String::from("hello");
    let _str5 = &str4;
    println!("str4: {} ,_str5: {}", str4, _str5);
    // 不可变引用不能在可变引用之后使用
    let str6 = &mut str4;
    // println!("str4: {} ,str5: {} ,str6: {}", str4, str5, str6);
    // println!("str4: {str4}");
    // 可变引用只能有一个，且在可变引用没有归还的时候，源变量不能使用
    println!("str6: {str6}");
    println!("str4: {str4}");

    println!("-------------------------slice-------------------------");
    let str7 = String::from("hello world");
    let str8 = &str7[..];
    println!("str8 : {str8}");
    let str9 = first_world(&str7);
    println!("str9 is {str9}");
    let len = str7.len();
    let str10 = first_world(&str7[..len]);
    println!("str10 is {str10}");
}

fn calculate_length(p0: String) -> (String, usize) {
    let len = p0.len();
    (p0,len)
}

fn give_ownership() -> String {
    let result = String::from("hello");
    result
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[..i]; }
    }
    &s[..]
}
