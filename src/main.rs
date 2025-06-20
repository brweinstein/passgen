mod cli;
mod generator;
mod random;
mod utils;

use cli::get_cli;
use generator::create_password;
use utils::{copy_to_clipboard, save_to_file, calculate_entropy};

fn main() {
    let cli = get_cli();
    let charset = cli.charset();

    let mut passwords = Vec::new();

    for _ in 0..cli.count {
        let pwd = create_password(cli.length, &charset, cli.no_repeat);
        passwords.push(pwd);
    }

    if cli.entropy {
        let entropy = calculate_entropy(cli.length, charset.len() as f64);
        println!("Estimated entropy per password: {:.2} bits", entropy);
    }

    let all_passwords = passwords.join("\n");

    let mut handled = false;

    if cli.clipboard {
        match copy_to_clipboard(&all_passwords) {
            Ok(_) => {
                if cli.count == 1 {
                    println!("Password successfully copied to clipboard");
                } else {
                    println!("{} passwords successfully copied to clipboard", cli.count);
                }
                handled = true;
            }
            Err(e) => eprintln!("Failed to copy to clipboard: {}", e),
        }
    }

    if let Some(ref path) = cli.save_path {
        match save_to_file(path, &all_passwords) {
            Ok(_) => {
                if cli.count == 1 {
                    println!("Password written to file at {}", path);
                } else {
                    println!("{} passwords written to file at {}", cli.count, path);
                }
                handled = true;
            }
            Err(e) => eprintln!("Failed to save to file: {}", e),
        }
    }

    if !handled {
        for pwd in passwords {
            println!("Password: {}", pwd);
        }
    }
}
