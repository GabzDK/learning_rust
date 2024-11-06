fn main() {
    
    // floats

    let _x = 2.0; // f34, douple precision

    let _y: f32 = 3.0; // f32, single precision floats
    
    let heart_eyed_cat = '😻';

    println!("{heart_eyed_cat}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("Value of tup is: {x},{y},{z}");


    // or we can access it with a . followed by the value that we want to access 
    
    let a: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = a.0;
    let six_point_four = a.1;
    let one = a.2;


    println!("{five_hundred}, {six_point_four}, {one}");


    // The tuple without any values has a special name, unit. 
    // This value and its corresponding type  are both written () and represent 
    // an empty value or an empty return type. 
    // Expressions implicitly return the unit value 
    // if they don’t return any other value.



    // we can modify individual elements of a mutable tuple.
    let mut x: (i32, i32) = (1, 2);

    x.0 = 0;
    x.1 +=5;


    // array
    //  If you dont know what to use, probably use a vector.
    //  arrays are more useful when you know the number of
    //  elements will not need to change.

    let _months = ["January", "February", "I am lazy to type"];
    
    // Declare array type, first the type then number or characters
    let _numbers: [i32;5] = [1,2,3,4,5];
    //You can also initialize an array to contain the same value for each element 
    //by specifying the initial value, followed by a semicolon, and then the 
    //length of the array in square brackets, as shown here:

    
    let a = [3; 5];

    // access array elements

    let first = a[0];
    let second = a[1];


}
