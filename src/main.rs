use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let apples = 5;
    let panios = 6;

    print!("{}", apples);
    print!("{}", panios);

    // let some_thing = String::from(guess);
    // println!("{}", some_thing);

    let secret_number = thread_rng().gen_range(1..109);

    println!("secret_number is {}", secret_number);

    loop {
        println!("guess some number!!!");
        println!("please input you guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match  guess.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => continue,
        };
        println!("you guess number: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("TOo small!!"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("you win!!");
                break;
            },
        }
    }
    let v = func();

    println!("function call return value: {}", v);
    println!("function call return value: {}", func1());
    println!("The result is {}", loop1());

    reference();
}


fn func() -> i32 {
    let x = 10;
    x + 10
}

fn func1 () -> u32 {
    return 10;
}

fn loop1() -> u32 {
    let mut number = 0;
    let result = loop {
        number += 10;

        if number == 20 {
            break number * 2;
        }
    };
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    return result;
}

fn reference() {
    let mut s = String::from("hello world");

    // let s1 = &s;
    // let s2 = &s;

    let s3 = &mut s;

    println!("{}, and ",  s3);
}



