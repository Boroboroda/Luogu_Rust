use std::io;
fn get_primes(n: i32) -> Vec<i32> { //埃拉托斯特尼筛法（Sieve of Eratosthenes）
    if n <= 2 {
        return Vec::new();
    }
    
    let mut sieve = vec![true; n as usize];
    sieve[0] = false;
    sieve[1] = false;
    
    for current in 2..=(f64::from(n).sqrt() as u32) {
        if sieve[current as usize] {
            let start = (current * current) as usize;
            for multiple in (start..n as usize).step_by(current as usize) {
                sieve[multiple] = false;
            }
        }
    }
    
    sieve.iter()
         .enumerate()
         .filter_map(|(i, &is_prime)| if is_prime { Some(i as i32) } else { None })
         .collect()
}

fn decomposation(n:i32){
    let mut primes=get_primes(n);
    primes.sort_by_key(|&num| std::cmp::Reverse(num));

    let mut i=0;

    loop{
        if n % primes[i]==0{
            println!("{}",primes[i]);
            break;
        }
        i+=1;
    }
}

fn main(){
    let mut input=String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut s=input.trim().split(' ');

    let n:i32=s.next().unwrap()
               .parse().unwrap();

    decomposation(n);
}