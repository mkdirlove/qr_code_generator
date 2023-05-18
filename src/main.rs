extern crate qrcode;
use qrcode::QrCode;
use qrcode::render::unicode;
use std::env;

fn clear_console() {
    if cfg!(windows) {
        print!("{}[2J", 27 as char);
    } else {
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let text = match args.iter().position(|arg| arg == "-t" || arg == "--text") {
        Some(index) => {
            if let Some(text) = args.get(index + 1) {
                text.clone()
            } else {
                println!("Please provide a value for the -t or --text flag.");
                return;
            }
        }
        None => {
            println!("Please provide a value for the -t or --text flag.");
            return;
        }
    };

    let code = QrCode::new(text.trim()).unwrap();

    let image = code.render::<unicode::Dense1x2>()
        .quiet_zone(false)
        .build();

    clear_console();
    println!("\n");
    println!("{}", image);
}
