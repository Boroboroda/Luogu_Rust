use std::io;

fn mom_bank(budget:Vec<i32>) -> f32 {
    let mut bank = 0;
    let mut current = 0;
    let mut i = 0;

    loop{
        if i == 12 {
            break;
        }
        current += 300 - budget[i] ;
        // println!("第{}个月，余额为：{}, 银行有: {}", i + 1, current, bank);
        
        if current < 0 {
            return - (i as f32 + 1.0);
        }

        if current >= 100 {
            bank += (current / 100) * 100;
            current -= current / 100 * 100;
        }
        i += 1;
    }
    
    return bank as f32 * 1.2 + current as f32;
}

fn main() {
    // let mut numbers = vec![];
    let mut i = 0;
    let mut numbers: Vec<i32> = vec![];
  
	loop{
        if i == 12{
            break;
        }
        let mut input=String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut s = input.trim().split(' ');
        // println!("{:?}",parsed);
        let num = s.next().unwrap().parse::<i32>().unwrap();
        numbers.push(num);
        i += 1
    }
    println!("{}", mom_bank(numbers) as i32);
}