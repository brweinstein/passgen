use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
struct Cli {
    length: usize,
    symbols: bool,
    numeric: bool,
    alpha: bool,
}

fn parse() -> Cli {
    let mut args = env::args().skip(1);

    let mut length = 16; //default value
    let mut symbols: bool = true;
    let mut numeric: bool = true;
    let mut alpha: bool = true;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-l" | "--length" => {
                if let Some(val) = args.next() {
                    length = val.parse().unwrap_or(16);
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
            _ => {
                eprintln!("{}: Invalid flag", arg.as_str());
                std::process::exit(1);
            }
        }
    }

    Cli{length, symbols, numeric, alpha}
}

fn get_chars(cli: Cli) -> Vec<char> {
    let mut charset = String::new();

    if cli.alpha {
        charset.push_str("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    if cli.numeric {
        charset.push_str("0123456789");
    }

    if cli.symbols {
        charset.push_str("!@#$%^&*()-_=+[]{};:,.<>?/|\\");
    }

    if charset.is_empty() {
        eprintln!("Error: No characters provided");
        std::process::exit(1);
    }

    charset.chars().collect()
}

fn create_password(length: usize, charset: Vec<char>) -> String {
    let mut arr: Vec<char> = vec!['a'; length];
    for i in 0..length {
        let random_char = charset[random_idx(charset.len())];
        arr[i] = random_char;
    }

    arr.iter().collect()
}

//See Linear Congruential Generator
fn lcg(seed: u64, max: u64) -> u64 {
    const MULTIPLIER: u64 = 984357762361238;
    const INCREMENT: u64 = 1;

    (MULTIPLIER.wrapping_mul(seed).wrapping_add(INCREMENT)) % max
}

fn random_idx(max: usize) -> usize {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seed = duration.as_nanos() as u64;
    lcg(seed, max as u64) as usize
}

fn main() {
    let cli = parse();
    let charset: Vec<char> = get_chars(cli.clone());
    let pwd = create_password(cli.length, charset.clone());

    println!("Password: {}", pwd)
}
