use std::env::var;
use std::sync::OnceLock;

pub fn address() -> &'static String {
    static ADDRESS: OnceLock<String> = OnceLock::new();
    match var("ACTIX_WEB_ADDRESS") {
        Ok(addr) => ADDRESS.get_or_init(|| addr),
        Err(e) => {
            eprintln!("Error: could not find env var ACTIX_WEB_ADDRESS {}", e);
            panic!()
        }
    }
}

pub fn port() -> &'static u16 {
    static PORT: OnceLock<u16> = OnceLock::new();
    match var("ACTIX_WEB_PORT") {
        Ok(port) => match port.parse::<u16>() {
            Ok(p) => PORT.get_or_init(|| p),
            Err(e) => {
                eprintln!("Error: failed to parse ACTIX_WEB_PORT to u16 {}", e);
                panic!();
            }
        },
        Err(e) => {
            eprintln!("Error: could not find env var ACTIX_WEB_PORT {}", e);
            panic!();
        }
    }
}

pub fn db_url() -> &'static String {
    static URL: OnceLock<String> = OnceLock::new();
    match var("DB_URL") {
        Ok(url) => URL.get_or_init(|| url),
        Err(e) => {
            eprintln!("Error: could not find env var DB_URL {}", e);
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_env::with_var;

    #[test]
    fn test_address() {
        with_var("ACTIX_WEB_ADDRESS", Some("127.0.0.1"), || {
            assert_eq!(address(), "127.0.0.1");
            assert_ne!(address(), "127.0.0.2");
        });
    }

    #[test]
    #[should_panic]
    fn test_address_panic() {
        with_var("ACTIX_WEB_ADDRESS", None as Option<&str>, || {
            address();
        });
    }

    #[test]
    fn test_port() {
        with_var("ACTIX_WEB_PORT", Some("8080"), || {
            assert_eq!(*port(), 8080);
            assert_ne!(*port(), 8081);
        });
    }

    #[test]
    #[should_panic]
    fn test_port_panic() {
        with_var("ACTIX_WEB_PORT", None as Option<&str>, || {
            address();
        });
    }

    #[test]
    fn test_db_url() {
        with_var("DB_URL", Some("sqlite://db.sqlite?mode=rwc"), || {
            assert_eq!(db_url(), "sqlite://db.sqlite?mode=rwc");
            assert_ne!(db_url(), ":D");
        });
    }

    #[test]
    #[should_panic]
    fn test_db_url_panic() {
        with_var("DB_URL", None as Option<&str>, || {
            db_url();
        });
    }
}
