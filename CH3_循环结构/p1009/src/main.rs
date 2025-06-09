use std::io;

fn fractal_sum(n:i64) -> i64{
    let mut sum=0;

    for i in 1..n+1{
        let mut cur_product = 1;
        for j in 1..i+1{
            cur_product*=j;
        }
        sum+=cur_product;
    }
    return sum;

}

fn main(){
    let mut input=String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut s=input.trim().split(' ');

    let a:i64=s.next().unwrap()
               .parse().unwrap();
    println!("{}",fractal_sum(a));
}