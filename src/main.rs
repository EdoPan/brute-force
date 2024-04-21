use std::io;
use std::io::Write;
use reqwest::blocking::Client;

fn handle_mode() -> (i32, String) {
    println!("
    __________                __           ___________                          __________                __    
    \\______   \\_______ __ ___/  |_  ____   \\_   _____/__________   ____  ____   \\______   \\__ __  _______/  |_  
     |    |  _/\\_  __ \\  |  \\   __\\/ __ \\   |    __)/  _ \\_  __ \\_/ ___\\/ __ \\   |       _/  |  \\/  ___/\\   __\\ 
     |    |   \\ |  | \\/  |  /|  | \\  ___/   |     \\(  <_> )  | \\/\\  \\__\\  ___/   |    |   \\  |  /\\___ \\  |  |   
     |______  / |__|  |____/ |__|  \\___  >  \\___  / \\____/|__|    \\___  >___  >  |____|_  /____//____  > |__|   
            \\/                         \\/       \\/                    \\/    \\/          \\/           \\/         
    -----------------------------------------------------------------------------------------------------------
    ");
    print!("Insert URL: ");
    let mut u = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut u).expect("Failed to read URL");
    println!("Choose mode: Low (1), Medium (2), High (3)");
    print!("Choose mode: ");
    io::stdout().flush().unwrap();
    let mut m = String::new();
    io::stdin()
        .read_line(&mut m).expect("Failed to read mode");
    let mode: i32 = m.to_string().replace("\n", "").parse().unwrap();
    let url: String = u.replace("\n", "");
    return (mode, url);
}

fn handle_low_mode(url: String) {
    let mut username = String::new();
    let mut password = String::new();
    print!("Insert username: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut username).expect("Failed to read username");
    print!("Insert password: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut password).expect("Failed to read password");
    //make_request(url, username, password)
}

// Fixed username, password fetched from a file
fn handle_medium_mode() {
    println!("Inside handle_medium_mode");
}

// Username fetched from a file, password fetched from a file
fn handle_high_mode() {
    /*  check username and all password 
    -> then move to check the second username and all password 
    -> so on
    */
    println!("Inside handle_high_mode");
}

fn make_request(url: String, u: String, p: String) {
    let client = Client::new();
    let result = client.post(url)
    .body("{\"username\":".to_string() + &u + "\"password\":" + &p + "}").send();

    if result.is_ok() {
        println!("{:#?}", result.ok().unwrap());
    }
    else if result.is_err() {
        println!("{:#?}", result.err());
    }
}

fn main() {
    let result = handle_mode();
    let url = result.1;
    match result.0 {
        1 => handle_low_mode(url),
        2 => handle_medium_mode(),
        3 => handle_high_mode(),
        _ => ()
    }
}