use std::env;


//TODO the 13 digits code that is actually used in comparison to this one
fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        return Err(String::from("Function needs exactly 1 argument"));
    }

    let s: String = args[1].replace("-", "");
    if s.len() != 10 {
        return Err(String::from("This string can't be an ISBN number"));
    }
    let mut sum: u32 = 0; 
    let mut curr = 10;

    for c in s.chars(){
        if c.is_ascii_digit(){
            sum += c.to_digit(10).unwrap() * curr;
            curr -= 1;
            continue;
        }
        if c == 'X'{
            sum += 10 * curr;
            curr -= 1;
            continue;
        }
        return Err(String::from("This string can't be an ISBN number"));
    }

    if sum % 11 == 0 {
        println!("This string is a valid ISBN number");
        return Ok(());
    }

    Err(String::from("This string is not a valid ISBN number"))
}
