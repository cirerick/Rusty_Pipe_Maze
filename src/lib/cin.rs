use std::io;

//getting different data types from user input
#[allow(unused)]
pub fn cin_string() -> String {
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).expect("error: invalid response.");

    //removing /n from input, not great, but should work for now.
    //should be checking for \n rather than assuming it will be at the end
    user_input = user_input[..(user_input.len() - 1)].to_string(); 

 
    return user_input
}

#[allow(unused)]
pub fn cin_char() -> char {
    let user_char: char;
    let user_temp_string: String = cin_string(); 

    if user_temp_string.chars().count() == 1 {
        let char_vec: Vec<char> = user_temp_string.chars().collect();
        user_char = char_vec[0];
        
        //user_char = user_temp_string.chars().next().unwrap();

    } else {
        println!("Not of char value.");
        return cin_char()
    }

    return user_char
}

#[allow(unused)]
pub fn cin_u8() -> u8 {
    let num: u8 = match cin_string().trim().parse() {
        Ok(parsed_num) => parsed_num,
        Err(_) => {
            println!("Not an integer."); 
            return cin_u8()
        },
    };

    return num
}

#[allow(unused)]
pub fn cin_i8() -> i8 {
    let num: i8 = match cin_string().trim().parse() {
        Ok(parsed_num) => parsed_num, 
        Err(_) => {
            println!("Not an integer or within scope.");
            return cin_i8()
        }
    };

    return num
}

#[allow(unused)]
pub fn cin_i16() -> i16 {
    let num: i16 = match cin_string().trim().parse() {
        Ok(parsed_num) => parsed_num,
        Err(_) => {
            println!("Not an integer.");
            return cin_i16()
        }
    };
    return num
}

#[allow(unused)]
pub fn cin_u16() -> u16 {
    let num: u16 = match cin_string().trim().parse() {
        Ok(parsed_num) => parsed_num,
        Err(_) => {
            println!("Not an integer."); 
            return cin_u16()
        },
    };

    return num
}

#[allow(unused)]
pub fn cin_u32() -> u32 {
    let num: u32 = match cin_string().trim().parse() {
        Ok(parsed_num) => parsed_num,
        Err(_) => {
            println!("Not an integer.");
            return cin_u32()
        },
    };

    return num
}

#[allow(unused)]
pub fn cin_usize() -> usize {
    let num = match cin_string().trim().parse() {
        Ok(parsed_num) => parsed_num,
        Err(_) => {
            println!("Not an integer.");
            return cin_usize()
        },
    };

    return num
}