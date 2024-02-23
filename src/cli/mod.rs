use std::{env, fs::File};

pub fn init() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        help_message();
    }
    match args[1].as_str() {
        "elf" => {}
        "raw" => load_from_raw(&args),
        _ => help_message(),
    }
}

fn load_from_raw(args: &Vec<String>) {
    let path: String = match args.get(2) {
        Some(inner) => inner.clone(),
        None => help_message(),
    };

    let raw_file = File::open(path).expect("Unable to find file.");
}

fn help_message() -> ! {
    println!(
        "    [*] elf - Load elf using default firmware

    [*] raw - loads raw disk image"
    );
    std::process::exit(1);
}
