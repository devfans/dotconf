//! # A super light-weight dotenv library
//! 
//! With less than 20 lines of code of the core part, but meet most of the requirements of a dotenv. JUST KEEP IT SIMPLE!
//!
//! ## Examples
//! 
//! ```
//! use dotconf::{init, init_with_path};
//! 
//! std::fs::write(".env", "a=b").unwrap();
//! init().expect("Failed to load env conf file (default: .env)");
//! 
//! std::fs::write(".dotenvfile", "
//!     a=b # This is a comment
//!     b=32
//!     c=true
//! ").unwrap();
//! init_with_path(".dotenvfile").expect("Failed to load from the specified env conf file");
//! 
//! // Read value with env::var with some simple type conversions
//! let a = dotconf::var("a").to_string().unwrap();
//! let b = dotconf::var("b").to_isize().unwrap();
//! let c = dotconf::var("c").to_bool().unwrap();
//! ```
//!
//!


use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env::{self, set_var, VarError};

/// Error type wrapped a String
#[derive(Debug, Clone)]
pub struct Error(String);
impl Error {
    pub fn to_string(self) -> String {
        self.0
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Load dotenv file with a default path.
/// Use `init_with_path` to load from a specific file.
///
/// # Examples
///
/// ```
/// use dotconf;
///
/// std::fs::write(".env", "a=b").unwrap();
/// dotconf::init().expect("Failed to load env conf file");
/// ```
pub fn init() -> Result<(), Error> { init_with_path(".env") }

/// Load dotenv file with a specified path.
///
/// # Examples
///
/// ```
/// use dotconf;
///
/// std::fs::write(".dotenv_another", "a=b").unwrap();
/// dotconf::init_with_path(".dotenv_another").expect("Failed to load env conf file");
/// ```
pub fn init_with_path(path: &str) -> Result<(), Error>{
    let pairs = parse_dotconf_file(path)?;
    for (k, v) in pairs {
        unsafe { set_var(k, v); }
    }
    Ok(())
}

/// Parse dotenv file as key-value pairs
/// use `#` to start a comment. 
/// 
/// Sample:
/// 
/// `url = https://xxxx.com  # Specify server address here`
/// 
pub fn parse_dotconf_file(path: &str) -> Result<Vec<(String, String)>, Error> {
    let path = Path::new(path);
    let file = File::open(path).map_err(|err| Error(err.to_string()))?;
    let reader = io::BufReader::new(file);
    let mut pairs = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(text) => {
                if let Some(line) = text.split('#').into_iter().next() {
                    if let Some((k, v)) = line.split_once('=') {
                        pairs.push((k.trim().to_string(), v.trim().to_string()));
                    }
                }
            }
            _ => {}
        }
    }
    Ok(pairs)
}

/// The wrapped env var result.
///
/// # Examples
///
/// ```
/// use dotconf;
///
/// std::fs::write(".env", "a=b").unwrap();
/// dotconf::init().expect("Failed to load env conf file");
/// dotconf::var("a").to_string();
/// dotconf::var("b").to_isize();
/// dotconf::var("c").to_bool();
/// ```
#[derive(Clone, Debug)]
pub struct Value(pub Result<String, VarError>);
impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.0 {
            Ok(ref s) => write!(f, "{}", s),
            Err(ref err) => write!(f, "{}", err),
        }
    }
}

/// Read key-value pair with `env::var`
/// 
/// # Examples
/// 
/// ```
/// dotconf::var("a").to_string().is_some();
/// dotconf::var("b").to_usize().is_none();
/// 
/// ```
/// 
pub fn var(key: &str) -> Value { Value(env::var(key)) }
impl Value {
    pub fn to_string(self) -> Option<String> {
        match self.0 {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn to_isize(self) -> Option<isize> {
        match self.0 {
            Ok(v) => {
                match v.parse::<isize>() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            },
            Err(_) => None,
        }
    }

    pub fn to_usize(self) -> Option<usize> {
        match self.0 {
            Ok(v) => {
                match v.parse::<usize>() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            },
            Err(_) => None,
        }
    }

    pub fn to_f64(self) -> Option<f64> {
        match self.0 {
            Ok(v) => {
                match v.parse::<f64>() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            },
            Err(_) => None,
        }
    }

    pub fn to_bool(self) -> Option<bool> {
        match self.0 {
            Ok(v) => {
                match v.to_lowercase().as_str() {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None,
                }
            },
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{init, var};

    #[test]
    fn it_works() {
        let raw = "
        a = hi
        b = -123
        c = false
        ";
        fs::write(".env", raw).unwrap();
        init().unwrap();
        assert_eq!(var("a").to_string(), Some("hi".to_string()));
        assert_eq!(var("b").to_isize(), Some(-123isize));
        assert_eq!(var("c").to_bool(), Some(false));

        fs::remove_file(".env").unwrap();
    }
}
