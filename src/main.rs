use std::io;

fn main() {
    println!("Fahrenheit to Celsius Conversion Formula. Enter F value:");

    loop {
        let mut user_input = String::new(); //returning an instance of F value
        io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

        let user_input:f64 = match user_input.trim().parse(){ //validation and runtime computation
            Ok(num) => f_to_c(num),
            Err(_) => continue,
        };
        println!("The converted C value is: {}", user_input);
        break;
    }
}

fn f_to_c(temp: i32)-> f64 {
    (temp - 32) as f64 * 0.5556
}
