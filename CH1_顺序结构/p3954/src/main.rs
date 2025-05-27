use std::io;
fn examination(a:i32, b:i32, c:i32) -> i32 {
    let result = (a as f64 * 0.2 + b as f64 * 0.3 + c as f64 * 0.5) as i32;

    return result;
}

fn main(){
    let mut input=String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut s=input.trim().split(' ');

    let a:i32=s.next().unwrap()
               .parse().unwrap();
    let b:i32=s.next().unwrap()
               .parse().unwrap();
    let c:i32=s.next().unwrap()
               .parse().unwrap();
    println!("{}",examination(a, b, c));
}