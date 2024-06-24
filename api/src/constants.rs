use std::env::var;
use std::sync::OnceLock;

pub fn address() -> &'static String {
    dotenv::dotenv().ok();
    static ADDRESS: OnceLock<String> = OnceLock::new();
    match var("ADDRESS") {
        Ok(addr) => {
            ADDRESS.get_or_init(|| addr)
        },
        Err(e) => {
            eprintln!("Error: could not find env var ADDRESS {}", e);
            panic!()
            // Could return a default &"localhost".to_string() here
        }
    }
}

pub fn port() -> &'static u16 {
    dotenv::dotenv().ok();
    static PORT: OnceLock<u16> = OnceLock::new();
    match var("PORT") {
        Ok(port) => match port.parse::<u16>() {
            Ok(p) => PORT.get_or_init(|| p),
            Err(e) => {
                eprintln!("Error: failed to parse PORT to u16 {}", e);
                panic!();
                // Could return a default 8080 here
            }
        },
        Err(e) => {
            eprintln!("Error: could not find env var PORT {}", e);
            panic!();
            // Could return a default 8080 here
        }
    }
}
