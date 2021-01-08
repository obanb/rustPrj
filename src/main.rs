//fn main() {
//    // println!("Hello, world!");
//    // println!("{}", highest(2, 4, 10));
//    // println!("{}", other(5, 10));
//    // loopto10();
//    // loopto10_2();
//    // whileloop();
//    // array_loop()
//    string1();
//}


// fns

fn highest(a: i32, b: u32, c: i8) -> i32 {
    let mut result = a;
    if b as i32 > result {
        result = b as i32;
    }
    if c as i32 > result {
        result = c as i32;
    }
    result
}

fn other(a: i32, b: i32) -> i32 {
    let mut c = a + b;
    c = c % 4;
    c = c / 2;
    c += 1;
    c
}

fn loopto10() {
    let mut n = 0;
    loop {
        n += 1;
        println!("loop");
        if n >= 10 {
            return;
        }
    }
}

fn loopto10_2() {
    for n in 0..10 {
        println!("loop for in");
        if n >= 10 {
            return;
        }
    }
}

fn whileloop() {
    let mut n = 0;

    while n < 10 {
        n += 1;
        println!("while");
    }
}

fn array_loop() {
    let mut v = Vec::new();
    v.push(4);
    v.push(6);
    v.push(7);

    for n in v {
        println!("edg in v: {}", n)
    }

    let nextv: Vec<i32> = vec![4, 8, 9, 10, 11, 22];

    for n in nextv {
        // if n % 2 == 0 {
        //     continue;
        // }
        if n == 11 {
            break;
        }
        println!("nextv: {}", n)
    }
}

fn array_loop2() {
    let arr: Vec<i32> = vec![4, 8, 9, 10, 11, 22];

    'outer: for i in 0..10 {
        for c in &arr {
            // borrowing?
            if c + i == 11 {
                break 'outer;
            }
            println!("nextv: {}", c)
        }
    }
}

//string

fn string1() {
    let s = String::from("Hello");

    for c in s.chars() {
        println!("{}", c)
    }
    for c in s.bytes() {
        println!("{}", c)
    }
}

//struct and methods

#[derive(Debug)]
struct User{
    name:   String,
    age: i32,
    height: i32,
    shoesize: u32
}

impl User{
    fn simple_string(&self)->String{
        format!("{} - {} - {}cm - shoe:{}", self.name, self.age, self.height, self.shoesize)
        //strednik rika ze pokracujeme, bez stredniku return
    }
    fn grow(&mut self, h:i32){
        self.height += h;
    }

    fn die(self){
        println!("self {}", self.simple_string());
    }
}

//fn main(){
//    let mut u = User{
//        name: "matt".to_string(),
//        height: 22,
//        age: 17,
//        shoesize: 12
//    };
//
//    println!("user is {:?}",u);
//    println!("user is {:?}",u.simple_string());
//    u.grow(20);
//    println!("user is {:?}",u.simple_string());
//    u.die();
//    u.grow(20);
//}

#[derive(Debug)]
pub struct Bed{
    size: i32,
    count: i32,
}

#[derive(Debug)]
pub enum Room{
    Kitchen(i32),
    Bedroom(Bed),
    Lounge,
}

fn main2(){
    use self::Room::*;

//    let t = Room::Kitchen(22);
////    let t2 = Room::Bedroom(Bed{count:22, size:33});
    let t = Kitchen(22);
    let t2 = Bedroom(Bed{count:22, size:33});
    let t3 = Bedroom(Bed{count:22, size:33});
    println!("Hello {:?}",t);

    match t{
        Kitchen(n) =>  println!("the room {}", n),
        d => println!("another match {:?}", d)
    }
    match t2{
       Kitchen(n) =>  println!("the room {}", n),
        d => println!("another match {:?}", d)
    }

    let v =  match t3{
        Kitchen(n) => n,
        _ => 0
    } + 10;

    println!("v is {}",v);
}


use std::collections::HashMap;
use std::env::args;

fn main(){
    println!("hello world");
    let mut hm = HashMap::new();
    hm.insert(3, "hello");
    hm.insert(5, "world");

    let r = match hm.get(&7){
        Some(v) => v,
        _=>"NOTHING",
    };

    let d = hm.get(&8).unwrap_or(&"empty");

    println!("mm {}",r);
    println!("mm {}",d);

    match "3t".parse::<f32>(){
        Ok(v) => println!("OK {}",v),
        Err(e) => println!("Error {}", e)
    }

    match get_arg(3){
        Ok(v) => println!("OK {}",v),
        Err(e) => println!("Error {}", e)
    }


}

fn get_arg(n: usize)-> Result<String, String>{
    for (i,a) in args().enumerate(){
        if i == n{
            return Ok(a);
        }
    }
    Err("not enough args".to_string())
}
// cargo run -- a r g s