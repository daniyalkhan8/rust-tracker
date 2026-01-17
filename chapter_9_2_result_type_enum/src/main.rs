use std::fs::File;
use std::{fs, io};
use std::io::{ErrorKind, Read};

fn read_user_name_from_file() -> Result<String, io::Error> {
    let mut username_str = String::new();
    File::open("username.txt")?.read_to_string(&mut username_str)?;

    Ok(username_str)
}

fn read_user_name_from_file_match () -> Result<String, io::Error> {
    let username_file_handler = File::open("username.txt");

    let mut username_file = match username_file_handler {
        Ok(file) => file,
        Err(error) => return Err(error)
    };

    let mut username_str = String::new();
    match username_file.read_to_string(&mut username_str) {
        Ok(_) => Ok(username_str),
        Err(error) => Err(error)
    }
}

fn get_user_name_form_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    let file_handler = File::open("text.txt");

    let mut file = match file_handler {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("text.txt") {
                Ok(created_file) => created_file,
                Err(e) => panic!("Problem creating file. {e:?}")
            },
            _ => panic!("Problem opening the file. {error:?}")
        },
    };

    let mut file_handler_2 = File::open("hello.txt").unwrap_or_else(|error| {
       if error.kind() == ErrorKind::NotFound {
           File::create("hello.txt").unwrap_or_else(|error| {
               panic!("Problem occurred while creating file {error:?}");
           })
       } else {
           panic!("Problem opening file {error:?}");
       }
    });

    let mut file1_str = String::new();
    file.read_to_string(&mut file1_str).unwrap_or_else(|error| {
        panic!("Couldn't read file {error:?}");
    });

    let mut file2_str = String::new();
    file_handler_2.read_to_string(&mut file2_str).unwrap_or_else(|error| {
        panic!("Couldn't read file {error:?}");
    });

    println!("{file1_str:?}");
    println!("{file2_str:?}");

    let username = read_user_name_from_file();
    let username = username.unwrap_or_else(|error| {
        error.to_string()
    });
    println!("{username}");

    let username_2 = read_user_name_from_file_match();
    let username_2 = username_2.unwrap_or_else(|error| {
        error.to_string()
    });
    println!("{username_2}");

    let hello_message = get_user_name_form_file();
    println!("{hello_message:?}");
}
