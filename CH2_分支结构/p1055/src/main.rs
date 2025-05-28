use std::io;

fn isbn_number(input: &String) {
    let mut numbers: Vec<i32> = vec![];

    for c in input.chars() {
    if c.is_digit(10){
            numbers.push(c.to_digit(10).unwrap() as i32);
        } else if c == 'X' {
            numbers.push(10);
        }
    }
    
    let mut sum: i32=0;
    
    for i in 0..numbers.len() as i32 - 1 {
        sum += numbers[i as usize] * (i + 1);
    }

    let check_digit = sum % 11;
    // Checking if the check digit is correct
    if check_digit == numbers[numbers.len() - 1] {
        println!("Right");
    } else {
        let part1 = numbers[0].to_string();
        let part2: String = numbers[1..4].iter().map(|n| n.to_string()).collect();  //make iter first, then map, then collect!
        let part3 :String = numbers[4..9].iter().map(|n| n.to_string()).collect();

        if check_digit == 10 {
            println!("{}-{}-{}-X", part1,part2,part3);
        }
        else {
            println!("{}-{}-{}-{}", part1,part2,part3,check_digit);
        }
    }
}

fn main(){
    let mut input=String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut s=input.trim().split(' ');

    let a:String=s.next().unwrap()
               .parse().unwrap();
    
    isbn_number(&a);
}