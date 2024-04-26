use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    //number_game();
    //date_type();
    // let rlt = user_login("admin", "123");
    // println!("rlt: {}", rlt);
    control();
}

/// 猜数字游戏
fn number_game() {
    loop {
        println!("Please input:");
        // 声明String类型变量
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        // 转换为i32类型
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };

        // 获取随机数
        let random_number = rand::thread_rng().gen_range(1..=10);
        println!("random_number: {},input: {}", random_number, input);
        match random_number.cmp(&input) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Greater => {
                println!("Too big!");
            }
        }
    }
}

fn date_type() {
    // 声明数组类型常量
    const ARRAYS: [i32; 5] = [1, 2, 3, 4, 5];
    // 使用索引的方式访问数组元素
    println!("{},{},{},{},{}", ARRAYS[0], ARRAYS[1], ARRAYS[2], ARRAYS[3], ARRAYS[4]);
    // 使用解构的方式访问数组元素
    let [a, b, _, d, ..] = ARRAYS;
    println!("{},{},{}", a, b, d);
    // 定义数组元素相同的数组
    const ARRAYS2: [i32; 5] = [3; 5];
    println!("{},{},{},{},{}", ARRAYS2[0], ARRAYS2[1], ARRAYS2[2], ARRAYS2[3], ARRAYS2[4]);

    // 声明元组类型
    let arrays: (i32, f32, &str, bool, char) = (1, 1.1, "admin", true, 'Y');
    // 使用索引的方式访问元组类型
    println!("{},{},{},{},{}", arrays.0, arrays.1, arrays.2, arrays.3, arrays.4);
    // 使用解构的方式访问元组元素
    let (a, b, ..) = arrays;
    println!("{},{}", a, b);
}

fn user_login(name: &str, pwd: &str) -> bool {
    if name.eq("admin") && pwd.eq("123") {
        return true;
    }
    false
}

fn control() {
    // 获取随机数
    let random_number = rand::thread_rng().gen_range(1..=3);
    if random_number == 1 {
        println!("{}", 1);
    } else if random_number == 2 {
        println!("{}", 2);
    } else {
        println!("{}", 3);
    }

    let mut index = 0;
    // loop循环
    loop {
        if index >= 5 {
            index = 0;
            break;
        }
        println!("index: {}", index);
        index += 1;
    }

    // while循环
    'l1: while true {
        while true {
            if index >= 5 {
                break 'l1;
            }
            println!("index: {}", index);
            index += 1;
        }
    }

    // for循环
    for i in 1..=9 {
        for j in 1..=i {
            print!("{j}*{i}={}\t", j * i);
        }
        println!();
    }
    let arrays = [1, 2, 3, 4, 5];
    for a in arrays {
        println!("{}", a);
    }
}
