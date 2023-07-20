use std::io;
fn main() {
    println!("Hello, world! This is my rust pracice.");
    first();
    second();
    third();
    fourth(29);
    fifth();
    parseCsv();
}

fn first(){
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");
}

fn second(){
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    let x = 5;
    let x= x +1;
    println!("The value of THREE_HOURS_IN_SECONDS is : {THREE_HOURS_IN_SECONDS}");
    println!("The value of x is : {x}");
    
    {
        let x = x*2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is : {x}");

    let spaces = "    ";
    println!("the spaces is {spaces}");

    let spaces = spaces.len();
    println!("the spaces is {spaces}");
    
    // let mut sspaces = "   ";
    // sspaces = spaces.len();
}

fn third(){
    let guess: u16 = "42".parse().expect("Not a number!");
    println!("Guess : {}", guess);

    let tup : (i16, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("The value of y is : {y}");

    let x: (i16, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let arr = [1,2,3,4,5];
    println!("plz enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("failed to read");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = arr[index];
    println!("value of element at index {index} is : {element}");

}

fn fourth(x : u32){
    println!("x is {x}");
}

fn fifth(){
    let y = {
        let x= 3;
        x+1
    };
    println!("The value of y is : {y}, {}", five());

}
fn five() -> i32{
    5
}

fn parseCsv(){
    let peuguin_data = "\
    common name, length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";
    let records = peuguin_data.lines();
    for ( i, record ) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(",").map(|field| field.trim()).collect();
        if cfg!(debug_assertions){
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}