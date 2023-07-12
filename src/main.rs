fn main() {
    println!("Hello, world! This is my rust pracice.");
    first();
    second();
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
