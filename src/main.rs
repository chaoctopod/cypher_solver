use std::io;

fn convert_using_cypher(i: String) -> String{
    // println!("Input value: {i}");
    match i.as_str() {
        "a" => "t".to_string(),
        "b" => "u".to_string(),
        "c" => "v".to_string(),
        "d" => "w".to_string(),
        "e" => "x".to_string(),
        "f" => "y".to_string(),
        "g" => "z".to_string(),
        "h" => "a".to_string(),
        "i" => "b".to_string(),
        "j" => "c".to_string(),
        "k" => "d".to_string(),
        "l" => "e".to_string(),
        "m" => "f".to_string(),
        "n" => "g".to_string(),
        "o" => "h".to_string(),
        "p" => "i".to_string(),
        "q" => "j".to_string(),
        "r" => "k".to_string(),
        "s" => "l".to_string(),
        "t" => "m".to_string(),
        "u" => "n".to_string(),
        "v" => "o".to_string(),
        "w" => "p".to_string(),
        "x" => "q".to_string(),
        "y" => "r".to_string(),
        "z" => "s".to_string(),
        _ => "?".to_string(),
    }
}
        
fn get_user_input() -> String {
    println!("Please input character to be deciphered.");
    
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input: &str = user_input.trim();

    println!("You entered: {user_input}");

    return user_input.to_string();

}

fn main() {
    loop {
        let user_input: String  = get_user_input();
        let converted: String = convert_using_cypher(user_input);
        println!("Converted: {converted}");
    }
}
