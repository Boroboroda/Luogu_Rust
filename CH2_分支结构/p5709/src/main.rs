use std:: io;
use std::cmp;

fn apples_prologue(m:i32, t:i32, s:i32) -> i32 {
    if t == 0{
        return 0;
    } else if s % t == 0  {
        return cmp::max(0, m - s/t)
    } else {
        // println!("{}", s/t);
        return cmp::max(0, m - s/t - 1)
    }
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
    println!("{}",apples_prologue(a, b, c));
}