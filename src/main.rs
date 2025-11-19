use std::io;
use std::env;

fn main() {

    // this is an approach to understand env::args,
    // cargo run -- "a text" should be used to actually see something as an arg here
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    loop{
        println!("Enter a text, enter q to exit the loop:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        //to ensure the whitespaces are removed before the input is processsed
        let input = input.trim();

        if input == "q" {break;}

        else{
            calculate_transform_key(input);
            continue;}

    }
}

fn calculate_transform_key(input: &str){
    let slices: Vec<i32> = input.chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap() as i32) // convert '0'..'9' -> 0..9
        .collect();

    println!("Numbers in the text: {:?}", slices);

    for i in 0..slices.len() - 1 {
        println!("x = {}, y = {} ", slices[i], slices[i + 1]);
        let or = slices[i] | slices[i + 1];
        let and = slices[i] & slices[i + 1];
        let xor = slices[i] ^ slices[i + 1];
        println!("Bitwise OR of {} ,  {} = {:0>4b}\n", slices[i], slices[i + 1], or);
        println!("Bitwise AND of {} ,  {} = {:0>4b}\n ", slices[i], slices[i + 1], and);
        println!("Bitwise AND of {} ,  {} = {:0>4b}\n ", slices[i], slices[i + 1], xor);
    }

}