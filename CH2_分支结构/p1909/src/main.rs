use std::io;
fn buy_pencil (input: &Vec<f64>) {
    let mut output = 0;

    let amount = input[0];

    let p1_amount = input[1];
    let p1_price = input[2];

    let p1_total = (amount / p1_amount).ceil() * p1_price;
    output = output.max(p1_total as i32);

    let p2_amount = input[3];
    let p2_price = input[4];

    let p2_total = (amount / p2_amount).ceil() * p2_price;
    output = output.min(p2_total as i32);

    let p3_amount = input[5];
    let p3_price = input[6];
    
    let p3_total = (amount / p3_amount).ceil() * p3_price;
    output = output.min(p3_total as i32);

    println!("{}", output as i32)

}

fn main() {
    // let mut numbers = vec![];
    let mut i = 0;
    let mut numbers = vec![];
    loop{
        if i == 4{
            break;
        }
        let mut input=String::new();
        io::stdin().read_line(&mut input).unwrap();
        let s: Vec<_> = input.trim().split(' ').collect();

        // 将字符串解析为 u32 类型的向量
        let parsed: Vec<f64> = s.iter()
            .filter_map(|x| x.parse::<f64>().ok())
            .collect();

        // println!("{:?}",parsed);
        numbers.extend(parsed);
        // numbers.extend(s .map(|x| x.parse::<f64>().unwrap())); 
        i += 1
    }
    buy_pencil(&numbers);
}