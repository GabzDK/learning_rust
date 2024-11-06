use std::io;

fn main() {

    let mut input = String::new();
    let mut input2  = String::new();
    let temperature: f64; 

    println!("Temperature converter!");
    println!("What do you want to convert? 'Celsius' or 'Fahrenheit' ? ");
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        println!("Insert a temperature: ");
        io::stdin()
            .read_line(&mut input2)
            .expect("failed to read line");
        let _input2: f64 = match input2.trim().parse(){
            Ok(num) => {
                temperature = num; 
                break;
            },
            Err(_) => {
                println!("Please input a valid number! ");
                input.clear();
                continue;
            }
        };
    }

    println!("{input}");
    println!("{temperature}");
    if input.trim().to_lowercase() == "celsius" {
        println!("Temperature is: {}F",calc_celsius(temperature));
    } else if input.trim().to_lowercase() == "fahrenheit"{
        println!("Temperature is: {}C",calc_fahrenheit(temperature));
    } else {
        println!("Please type a valid temperature.");
    }
}
fn calc_celsius(x:f64) -> f64 {
    (x * 9.0 / 5.0 ) + 32.0
}

fn calc_fahrenheit(x:f64) -> f64 {
    (x - 32.0 ) * 5.0 / 9.0
}
