extern crate rand;

use std::io;
use rand::Rng;
fn main() {
//-------------------------var----------------------------------
    println!("===========var test===========");
    // Variables can be type annotated.
    let _logical: bool = true;

    let _a_float: f64 = 1.0;  // Regular annotation
    let _an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let _default_float   = 3.0; // `f64`
    let _default_integer = 7;   // `i32`

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed.
    mutable = 11;

    // Variables can be overwritten with shadowing.
    let mutable = true;

    let _char : char = '哦';

    println!("char = {}", _char);

    let a = [1, 2, 3]; // a: [i32; 3]
    let mut m = [1, 2, 3]; // m: [i32; 3]
    m[1]=2;
    let a = [3; 20];

    println!("a size = {}", a.len());
    println!("a[19] = {}", a[19]);

    //diverges();
//-------------------------fun----------------------------------
    println!("===========fun test===========");
    let value = print_number(5, 10);
    println!("value = {}", value);

    let test_fun:fn(i32, i32)->i32 = print_number;
    let value = test_fun(3,5);
    println!("test_fun value = {}", value);

//-------------------------Slicing----------------------------------
    println!("===========Slicing test===========");
    let array = [1,2,3,4,5,6];

    let complete = &array[..];
    print_array(complete);

    let middle = &a[1..3];

//-------------------------Tuples----------------------------------
    println!("===========Tuples test===========");
    let x = (1, 2, "hello");
    let y = x;
    let (a, b, c) = y;
    println!("c = {}", c);

    let tuple = (1, 2, 3);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("x is {}", x);

//-------------------------while----------------------------------
    println!("===========while test===========");
    test_while();
    test_for();
    test_iterators();
    test_continue();
    test_loop_labels();

//-------------------------vectors----------------------------------
    println!("===========vectors test===========");
    let mut v = vec![1,2,3,4,5];

    let i: usize = 0;
    let j: i32 = 0;

    // Works:
    v[i];

    // Doesn’t:
    //v[j]; //error must usize

    println!("The third element of v is {}", v[2]);

    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }

    test_for_vec(&v);
    v[0] = 10;
    test_for_vec(&v); //use & can ues v

    let y = vec![1;10];
    println!("The init element of y is {}", y[2]);
//-------------------------ownership----------------------------------
    println!("===========ownership test===========");
    let v = vec![1,2,3,4,5];
    let v2 = v;
    println!("The owner v element of v is {}", v2[2]); //v is not used!!!

    let v = 4;
    let v2 = v;
    println!("copy to v2 ,then v = {}", v);
//-------------------------&mut----------------------------------
    println!("===========&mut test===========");
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);

//-------------------------game----------------------------------
    println!("===========game test===========");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

//        let guess: i32 = guess.trim().parse()
//            .expect("Please type a number!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

use std::cmp::Ordering;

fn print_number(x: i32, y: i32) -> i32{
    println!("x is: {}", x+y);
    let value = y-x;
    if value >0 {
        y-x
    } else {
        x-y
    }
}

fn print_array(array:&[i32]) {
    let size = array.len();
    println!("array size = {}", size);
//    let i = 0;
    for i in array {
        print!("{}, ", i);
    }
    println!("");
}

fn diverges() -> ! {
    panic!("This function never returns!");
}

fn test_while() {
    let mut index = 0;
    while index < 10 {
        index = index + 1;
        println!("index = {}", index);
    }
}

fn test_for() {
    for (index, value) in (5..10).enumerate() {
        println!("index = {}, value = {}", index, value);
    }
}

fn test_iterators() {
    let lines = "hello \n world! \n haha".lines();
    for (linenumber, line) in lines.enumerate() {
        println!("linenumber = {}; line = {}", linenumber, line);
    }
}

fn test_continue() {
    for x in (0..11) {
        if x%2 == 0 {
            continue
        }

        println!("test_continue x = {}", x);
    }
}

fn test_loop_labels() {
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
            if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
            println!("x: {}, y: {}", x, y);
        }
    }
}

fn test_for_vec(v:&Vec<i32>) {
    for i in v {
        println!("A reference to {}", i);
    }
}