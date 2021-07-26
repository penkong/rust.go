

// const always immutable
const MAX_IT: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);
    println!("{}",MAX_IT);

    // shadowed var
    // By using let, we can perform a few transformations on a value 
    // but have the variable be immutable after those transformations
    let y = 5;
    let y = y + 1;
    let y = y + 2;
    println!("{}", y);

    // change type by shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    // error case of changing type
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // explicity type annotation
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // Scaler type : represent single value .
    // int , floating - points , numbers , booleans , characters ,
    // -(2^(n-1)) <i< (2^(n-1) -1), 0 <u< 2^(n) - 1  (unsigned)
    // arch isize , usize --> dependes on type of computer you are in 
    // i32 is fastest 
    // iszie or usize use for indexing some sort of collection .

    // release mode does not contain panic mode for nummbers overflow  
    // Relying on integer overflow’s wrapping behavior is considered an error.

    // floating points 
    // f32 (single precision), f64(double precision) , default is f64
    // let x = 2.0;
    // let x: f32 = 2.44;

    // booleans , bool , true , false

    // Chracter type with single quote , 
    // 4bytes size and a Unicode Scalar Value , means more than ascii .

    //  ---------------

    // Compaund Types 
    // rust hast 2 primitive compound types

    // Tuple : number of values with variaty of types , fixed length
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destrcutre 
    let (x, y, z) = tup;
    // access
    let second = tup.1;

    // Array : same type , fixed length , sit on stack
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // with init , same value for x time
    let a = [3; 5];
    // access
    let first = a[0];
    // if use access more than length rust will panic on RUNTIME
    // in rust you can not access invalid memory access .

    // std lib has vector type let grow and shrink
}


fn another_function() {
    println!("Another function.");
}