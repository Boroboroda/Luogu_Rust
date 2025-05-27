use std::io;
fn elephant_water(a:i32, b:i32) -> i32{
    let pi = 3.14;  
    let volume = (b * b) as f64 * pi * a as f64;

    let t = (20000.0 / volume).ceil() as i32;
    return t;
}

fn main(){
    let mut input=String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut s=input.trim().split(' ');

    let a:i32=s.next().unwrap()
               .parse().unwrap();
    let b:i32=s.next().unwrap()
               .parse().unwrap();
    println!("{}", elephant_water(a, b));
}