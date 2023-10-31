// 派生注解
#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    age: u8,
    email: String,
    password: String,
}

#[derive(Debug)]
struct UserEx {
    active: bool,
    age: u8,
}

fn main() {
    println!("---------structure exercise----");
    let user1 = User {
        active: true,
        name: String::from("John"),
        age: 24,
        email: String::from("aaaa@qq.com"),
        password: String::from("qqqqqqqqq"),
    };
    println!("-------user1 :{:?}", user1);
    let mut user2 = User {
        ..user1
    };
    user2.email = String::from("qqqqqqq@qq.com");
    println!("-------user2:{:?}", user2);


    let user_ex1 = UserEx {
        active: true,
        age: 33,
    };

    let mut user_ex2 = UserEx {
        ..user_ex1
    };

    user_ex2.age = 88;
    // 基础数据类型，不会产生clone，而是copy
    println!("--------user_ex1 :{:?},user_ex2:{:?}", user_ex1, user_ex2);

    let user_ex3 = bind_user_ex(false, 99);
    println!("--------user_ex3 :{:?}", user_ex3);

    let point1 = Point(-10, 100);
    println!("----玩家坐标是:{:?}", point1);


    println!("------------------使用元组传递参数-------------------");
    let area_param = (10, 9);
    let area_result = area_cal(area_param);
    println!("------二元数组来进行传参,结果是:{}", area_result);

    println!("----------------使用结构体来计算");

    let rectangle = Rectangle {
        width: 8,
        height: 7,
    };

    let area_result2 = area_cal2(&rectangle);
    println!("width:{}", &rectangle.width);
    println!("height:{}", &rectangle.height);
    println!("------使用结构体来进行传参,结果是:{}", area_result2);


    println!("------使用结构体来进行传参,结果是:{}", area_result2);

    let area1 = rectangle.area();
    println!("-----area1:{},w:{}", area1, rectangle.height);

    let area2 = Rectangle::area_from(6, 7);
    println!("-----area2:{}", area2);
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        &self.width * &self.height
    }

    fn area_from(w: u32, h: u32) -> u32 {
        w * h
    }
}

fn area_cal(double_tuple: (u32, u32)) -> u32 {
    double_tuple.0 * double_tuple.1
}

fn area_cal2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


fn bind_user_ex(active: bool, age: u8) -> UserEx {
    UserEx {
        active,
        age,
    }
}

#[derive(Debug)]
struct Point(i32, i32);