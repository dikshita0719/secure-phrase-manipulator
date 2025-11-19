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
        let trimmed_input = input.trim();

        if input == "q" {break;}

        else{
            //Phase 1
            calculate_transform_key(trimmed_input);

            //Phase 2
            let length = input.len() ;
            println!("Before Move: ");
            println!("Stack Address of input(String -> non-primitive: {:p} \n Heap Content: {:#?}", &input, input.as_ptr());
            println!("Stack Address of Length(usize -> primitive): {} \n Content: {}", &length, length);

            let (new_input, new_length) = transform_phrase(input,length );

            println!("After Move: ");
            println!("Stack Address of New Input(String -> non-primitive): {:p} \n Heap Content: {:#?}", &new_input, new_input.as_ptr());
            println!("Stack Address of New Length(usize -> primitive): {} \n Content: {}", &new_length, new_length);


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
        println!("Bitwise XOR of {} ,  {} = {:0>4b}\n ", slices[i], slices[i + 1], xor);
    }

}

fn transform_phrase(input: String, key: usize ) -> (String, usize) {
    let mut new_input = input.trim_end().to_string();
    println!("Your Input:{}\nInput length: {}\n", new_input, key);
    new_input.push_str(&format!("_Hello❤ {} ", key).as_str());
    let len = new_input.len();

    println!("Following tuple will be used from now one: ({} , {})", new_input, len);

    (new_input, len )
}