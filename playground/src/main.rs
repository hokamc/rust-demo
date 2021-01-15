mod config;
mod models;
mod routes;

use playground::house::people::add;
use std::collections::HashMap;

fn five(x: i32) -> i32 {
    let y = {
        let z = 3;
        z + x
    };

    y
}

struct User {
    username: String,
    email: String,
    nickname: Option<String>,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn from(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
enum Message {
    Add(AddMessage),
    Remove(RemoveMessage),
    Change(ChangeMessage),
}

impl Message {
    fn print(&self) -> String {
        match self {
            Message::Add(_) => String::from("add"),
            Message::Remove(_) => String::from("remove"),
            Message::Change(_) => String::from("change"),
        }
    }
}

#[derive(Debug)]
struct AddMessage {
    value: u32,
}

#[derive(Debug)]
struct RemoveMessage {
    value: u32,
}

#[derive(Debug)]
struct ChangeMessage {
    value: u32,
}

fn match_u8(num: u8) {
    match num {
        1 => println!("1"),
        _ => println!("other num"),
    }
}

fn main() {
    v2();
}

fn v2() {
    let add = |a: i32, b: i32| {
        return a + b;
    };

    println!("{}", add(3, 4));
}

fn v1() {
    let mut x = 5;
    println!("{}", x);

    x = 10;
    println!("{}", x);

    let x = x;

    println!("{}", x);

    let spaces = "     ";
    let spaces = spaces.len();

    println!("{}", spaces);

    let guess: u32 = "32".parse().expect("u32 expected");

    println!("{}", guess);

    for x in 0..10 {
        println!("{}", x);
    }

    let tup: (i32, i64, i128) = (1, 2, 3);
    println!("{},{},{}", tup.0, tup.1, tup.2);

    let arr: [i128; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    println!("{}", five(123));

    let x = 3;
    if x < 5 {
        println!("bigger {}", x);
    } else {
        println!("smaller {}", x);
    }

    let x = "String";
    let x: String = x.chars().rev().collect();
    println!("{}", x);

    let x = String::from("asd");
    let x = borrow_sth(x);
    println!("{}", x);

    reference(&x);

    println!("{}", x);

    let user = User {
        username: String::from("email"),
        email: String::from("email"),
        nickname: None,
    };

    println!("{}", user.email);

    let rect = Rectangle {
        width: 10,
        height: 15,
    };
    println!("{}", area(&rect));

    println!("{:?}", rect);

    println!("{}", rect.area());

    println!("{:?}", rect);

    println!("{:?}", Rectangle::from(12));

    let m = Message::Add(AddMessage { value: 123 });

    println!("{:?}", m);

    println!("{}", m.print());

    match_u8(1);
    match_u8(2);

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    println!("{}", v[2]);

    for vl in &v {
        if vl > &0 {
            println!("hi");
        }
    }

    let mut str = String::from("hello");
    str.push_str("world");

    let s = &str[0..4];
    println!("{}", s);

    for c in str.chars() {
        println!("{}", c)
    }

    let mut map = HashMap::new();

    map.insert(String::from("Blue"), 10);

    println!("{}", map.iter().count());

    add();

    config::print_config();

    routes::health_route::print_health_route();

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", largest(&arr));

    // println!("{}", generic_largest(&arr));

    let mut map: HashMap<NumberView, &str> = HashMap::new();
    print_hashmap(&mut map);

    // panic!("Crash");

    let article = Article {
        headline: String::from("headline"),
        content: String::from("content"),
    };

    let a = if article.headline == "headline1" {
        "1"
    } else if article.headline == "headline2" {
        "2"
    } else {
        "3"
    };

    println!("{}", a);

    println!("{}", article.summarize());

    sum(&article);

    let r;

    r = {
        let x = 5;
        x
        // r = create_sth();
    };

    println!("{}", r);
}

fn create_sth() -> i32 {
    5
}

fn borrow_sth(sth: String) -> String {
    println!("{}", sth);
    sth
}

fn reference(sth: &String) {
    println!("{}", sth);
}

fn print_hashmap(map: &mut HashMap<NumberView, &str>) {
    map.insert(NumberView::ONE, "ASD");
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum NumberView {
    ONE,
    TWO,
    THREE,
}

impl NumberView {
    fn value(&self) -> String {
        match *self {
            NumberView::ONE => String::from("One"),
            NumberView::TWO => String::from("Two"),
            NumberView::THREE => String::from("Three"),
        }
    }
}

// fn generic_largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for value in list {
//         if value > largest {
//             largest = value;
//         }
//     }
//     largest
// }

fn largest(list: &[i32]) -> i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    println!("{}", NumberView::ONE.value());

    *largest
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub headline: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, {}", self.content, self.headline)
    }
}

pub fn sum(item: &impl Summary) {
    println!("{}", item.summarize());
}

pub fn sum2<T>(item: &T)
where
    T: Summary,
{
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(2 + 2, 4);
    }
}
