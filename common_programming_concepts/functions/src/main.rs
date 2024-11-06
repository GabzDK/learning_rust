fn main() {
    println!("Hello, world!");

    another_function(2);


    let x = {
        // THIS IS AN EXPRESSION!!!
        let y = 6;
        y + 2
     };

    println!("value of y: {x}");

    let five = five();
    println!("{five}");

    let plus = plus_one(6);
    println!("{plus}");

}

fn another_function(z: u8){
    println!("Value of x: {z}.");
}

fn five() -> i32{
    // this is only a five without a ; because this
    // makes it an expression, then we can return 
    // the value 5.
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
