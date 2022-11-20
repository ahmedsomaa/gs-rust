fn main() {
    // integers
    let num = 5;
    let negative: i8 = -2;

    println!("The value of num is {}, and the value of negative is {}", num, negative);

    // floats
    let x = 5.0;            // single precision
    let y: f64 = 25.0;      // double precision

    // mathematical operantion
    let addition = x + y;
    let subtraction = 10 - num;
    let multiplication = 2 * 5;
    let division = 30 / 5;
    let remainder = 30 % 4;

    println!("Addition value = {addition}");
    println!("Subtraction value = {subtraction}");
    println!("Multiplication value = {multiplication}");
    println!("Division value = {division}");
    println!("Remainder value = {remainder}");

    // tuple
    let tup: (u8, char, bool) = (1, '2', true);

    // use destructuring to access values
    let (n, c, b) = tup;

    println!("The value of n = {}, c = {}, b = {}", n, c, b);

    // or using the dot notation followed by the tuple index
    println!("First tuple element = {}, second = {}, third = {}", tup.0, tup.1, tup.2);

    // arrays
    let arr = [1, 2, 3, 4, 5];
    let list : [char; 5] = ['A', 'h', 'm', 'e', 'd'];
    let init = [3; 3];
    
    println!("Simple array value = {}", arr[0]);
    println!("Array with static type and size value = {}", list[2]);
    println!("Array with initialized value = {}", init[1]);
}
