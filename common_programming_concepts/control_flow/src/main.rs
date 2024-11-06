fn main() {
    // let number = 5;
    let condition = false;

    if condition  {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // If is an expression then we can use it to 
    // assign to a variable
    let number = if condition { 5 } else { 6 };
    println!("{number}");


    // there is 3 types of loops, "loop", "while" and "for"

    // infinite loop until you explicitly stop it.
    // loop {
    //     println!("again!");
    // }
    let mut counter = 0;

    let result = loop {
        counter +=1;

        if counter == 10{
            break counter * 2;
        }
    };
    println!("The result is: {result}");

    let mut count = 0;
    // you declare a label with ' but you dont need to.
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -=1;
        }
        count +=1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -=1;
    }
    println!("LIFTOFF!!!");


    // you can use while to loop over elements of a collection
    // such as an array.

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // this is not really good a more concise way to do this
    // is using a for loop.
    // using a while loop like this is slow and can cause
    // pain.

    for element in a {
        println!("LOOK A FOR LOOP: {element}")
    }

    // you can also use for loop to make a 
    // countdown by using .rev to reverse it
    for number in (1..4).rev(){
        println!("{number}!");
    }
    println!("FOR POWER");


}
