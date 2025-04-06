use rand::Rng;
use std::{env, fmt::Error};


const LOWERCASE: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

const UPPERCASE: [char; 26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

const DIGITS: [char;10] = ['0','1','2','3','4','5','6','7','8','9'];

const SPECIAL: [char; 33] = [' ','!','"','#','$','%','&','\'','(',')',
'*','+',',','-','.','/',':',';','<','=',
'>','?','@','[','\\',']','^','_','`','{','|','}','~'];

const CHARSET_ARRAY: [&[char];4] = [&LOWERCASE,&UPPERCASE,&DIGITS,&SPECIAL];

fn main() -> Result<(),()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Function needs at least 1 argument");
        return Err(());
    }
    let i = match args[1].parse::<usize>() {
        Ok(p) => p,
        Err(_) => {println!("1st function argument needs to be a number"); return Err(());}
    };
    let password: String = generate_password(i, args);

    if password.len() != i{
        println!("Generated password is not in the specified length");
        return Err(());
    }

    println!("{}", password);

    Ok(())
}


fn generate_password(n:usize,charset: Vec<String>) -> String{
    let mut rng = rand::rng();
    let mut char_sets: Vec<&[char]> = Vec::new();
    let mut set_mask: [bool; 4] =[false;4];
    let mut password: Vec<char> = Vec::new();

    for w in charset {
        match w.as_str(){
            "lowercase" => set_mask[0] = true,
            "uppercase" => set_mask[1] = true,
            "digits" => set_mask[2] = true,
            "special" => set_mask[3] = true,
            _ => continue
        }
    }

    for i in 0..4{
        if set_mask[i] {char_sets.push(CHARSET_ARRAY[i]);}
    }

    if char_sets.len() == 0 {
        for i in 0..4{
            char_sets.push(CHARSET_ARRAY[i]);
        }
    }

    for _ in 0..n{
        let i = rng.random_range(0..char_sets.len());
        let j = rng.random_range(0..char_sets[i].len());
        password.push(char_sets[i][j]);
    }

    password.into_iter().collect()
}
