fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    let i: i32 = 0b1100_1111_0000; //3312
    let i: i32 = i * 98_222;
    println!("{i}");
    // let mut spaces = "   ";
    // spaces = spaces.len();   //error

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}");
    let a = [1, 2, 3, 4, 5];
    // let index = 10;
    // let element = a[index];  //error
    // println!("The value of element is: {}", element);
}