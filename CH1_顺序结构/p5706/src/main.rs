use std::io;
fn share_coke(a:f64, b:i32) -> (String, i32) {
    let cups = b * 2;

    let c = format!("{:.3}", a/b as f64);

    return (c, cups);
}

fn main(){
    let mut input=String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut s=input.trim().split(' ');

    let a:f64=s.next().unwrap()
               .parse().unwrap();
    let b:i32=s.next().unwrap()
               .parse().unwrap();
    
    let (c,cups) = share_coke(a, b);
    println!("{}",c);
    println!("{}",cups);
}