use std::env;
use std::process;

#[derive(Clone)]
pub struct Cli {
   pub length: usize,
   pub symbols: bool,
   pub numeric: bool,
   pub alpha: bool,
   pub count: usize,
   pub clipboard: bool,
   pub no_repeat: bool,
   pub save_path: Option<String>,
   pub exclude: Vec<char>,
   pub entropy: bool,
}

impl Cli {
   pub fn charset(&self) -> Vec<char> {
      let mut charset = String::new();
      if self.alpha {
         charset.push_str("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
      }
      if self.numeric {
         charset.push_str("0123456789");
      }
      if self.symbols {
         charset.push_str("!@#$%^&*()-_=+[]{};:,.<>?/|\\");
      }
      let mut chars: Vec<char> = charset.chars().collect();
      chars.retain(|c| !self.exclude.contains(c));
      if chars.is_empty() {
         eprintln!("Error: No characters provided.");
         std::process::exit(1);
      }
      chars
   }
}

pub fn get_cli() -> Cli {
   let mut args = env::args().skip(1);

   let mut length = 16;
   let mut symbols = true;
   let mut numeric = true;
   let mut alpha = true;
   let mut count = 1;
   let mut clipboard = false;
   let mut no_repeat = false;
   let mut save_path = None;
   let mut exclude = vec![];
   let mut entropy = false;

   while let Some(arg) = args.next() {
      match arg.as_str() {
         "-l" | "--length" => {
            if let Some(val) = args.next() {
                  match val.parse::<usize>() {
                     Ok(parsed) => length = parsed,
                     Err(_) => {
                        eprintln!("Error: Invalid value for length: '{}'", val);
                        process::exit(1);
                     }
                  }
            } else {
                  eprintln!("Error: Missing value after '{}'", arg);
                  process::exit(1);
            }
         }
         "-an" | "--alphanumeric" => {
               symbols = false;
         }
         "-a" | "--alpha" => {
               symbols = false;
               numeric = false;
         }
         "-n" | "--numeric" => {
               alpha = false;
               symbols = false;
         }
         "-c" | "--count" => {
               if let Some(val) = args.next() {
                  count = val.parse().unwrap_or(1);
               }
         }
         "--clipboard" | "--copy" | "-cp" => clipboard = true,
         "--no-repeat" | "-nr" => no_repeat = true,
         "--save" | "-s" => {
               if let Some(val) = args.next() {
                  save_path = Some(val);
               } else {
                  eprintln!("No path provided for -s or --save flag");
                  std::process::exit(1);
               }
         }
         "--exclude" | "-ex" => {
               if let Some(val) = args.next() {
                  exclude = val.chars().collect();
               } else {
                  eprintln!("No excluded characters provided for -ex or --exclude flag")
               }
         }
         "--entropy" | "-en" => {
            entropy = true;
         }
         _ => {
               eprintln!("Invalid flag: {}", arg);
               std::process::exit(1);
         }
      }
   }

   Cli {
      length,
      symbols,
      numeric,
      alpha,
      count,
      clipboard,
      no_repeat,
      save_path,
      exclude,
      entropy,
   }
}
