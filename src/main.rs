mod hunter;
use std::collections::HashMap;

use hunter::errors::HunterError;
mod requestbin;
mod forms;
mod parsing;
use crate::hunter::HunterClient;
use std::io;

fn get_user_line() -> String {
    let stdin = io::stdin();
    let input = &mut String::new();
    let _ = stdin.read_line(input);
    input.trim().to_string()
}

#[tokio::main]
async fn main() {
    let mut client = match HunterClient::new() {
        Ok(v) => {v},
        Err(e) => {
            eprintln!("Error while creating client: {}", e.value);
            std::process::exit(1);
        }
    };
    println!("Please add exery URL in the scope of your hunt and then type \"done\".");
    loop {
        let input = get_user_line();
        if input == String::from("done") {
            break;
        }
        match client.scope(input) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error while adding URL to scope: {}", e.value);
                std::process::exit(2);
            }
        };
    }
    println!("Please select the current target/starting point (should be in scope).");
    match client.set_target(get_user_line()) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error while setting target: {}", e.value);
            std::process::exit(3);
        }
    };
    println!("To handle requests coming back from the vulnerable fields, we need a request bin.");
    println!("Should it be public (public internet, using requestbin.cn),");
    println!("or private (private networks, using your local IP) ? (pub/pv)");
    loop {
        let input = get_user_line();
        if input == String::from("pub") {
            match client.set_public_request_bin().await {
                Ok(_) => {
                    break;
                },
                Err(e) => {
                    eprintln!("Error while creating public bin: {}", e.value);
                    std::process::exit(4);
                }
            };
        }
        if input == String::from("pv") {
            println!("Please select your preferred private IP:");
            let ip = get_user_line();
            println!("Please select your preferred port:");
            let port: u16 = match get_user_line().parse() {
                Ok(v)=>{v},
                Err(e)=>{
                    eprintln!("Error while parsing port value: {}", e);
                    std::process::exit(5);
                }
            };
            match client.set_private_request_bin(ip, port.into()) {
                Ok(_) => {
                    break;
                },
                Err(e) => {
                    eprintln!("Error while creating private bin: {}", e.value);
                    std::process::exit(6);
                }
            };
        }
        println!("Please type \"pub\" or \"pv\"");
    }
    println!("Would you like to set up a custom cookie ? Session cookies can give us access to more pages (yes/no)");
    loop {
        let input = get_user_line();
        if input == String::from("no") {
            break;
        }
        if input == String::from("yes") {
            println!("Cookie value (if unsure type console.log(document.cookie) in your browser console) (must be URL-Encoded) :");
            match client.set_cookies(get_user_line()) {
                Ok(_) => {
                    break;
                },
                Err(e) => {
                    eprintln!("Error while setting Cookie: {}", e.value);
                    std::process::exit(7);
                }
            }
        }
        println!("Please type \"yes\" or \"no\"");
    }
    println!("Would you like to set up a custom User-Agent ? (yes/no)");
    loop {
        let input = get_user_line();
        if input == String::from("no") {
            break;
        }
        if input == String::from("yes") {
            println!("User-Agent value :");
            match client.set_user_agent(get_user_line()) {
                Ok(_) => {
                    break;
                },
                Err(e) => {
                    eprintln!("Error while setting User-Agent: {}", e.value);
                    std::process::exit(8);
                }
            }
        }
        println!("Please type \"yes\" or \"no\"");
    }
    println!("==== ALL SET ====");
    println!("Let's crawl the target to search for pages and forms, which may be vulnerable");
    println!("Select the recursive depth for crawling (recommended: 3) :");
    let depth: usize = match get_user_line().parse() {
                Ok(v)=>{v},
                Err(e)=>{
                    eprintln!("Error while parsing depth value: {}", e);
                    std::process::exit(9);
                }
            };

    match client.crawl(depth).await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error while crawling target: {}", e.value);
            std::process::exit(10);
        }
    };
    println!("[+] {} forms found, sending payloads everywhere...", client.known_forms().len());
    match client.track_all_forms_and_wait().await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error while sending payloads and listening: {}", e.value);
            std::process::exit(11);
        }
    };
    println!("Here are all known vulnerable fields at this point:");
    println!("{:#?}", client.known_vulnerable_fields);
    std::process::exit(0);
}

