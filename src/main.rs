mod opts;
use colored::Colorize;
use opts::{Opts, Quality};
use rand::Rng;

/// All available keys
const KEYS: [&str; 7] = ["A", "B", "C", "D", "E", "F", "G"];
/// Sharp symbol
const SHARP: &str = "#";
/// Flat symbol
const FLAT: &str = "♭";

fn main() {
    println!("{}", "Super Awesome Music Key Helper Tool".blue());

    let opts: Opts = argh::from_env();
    let key = create_random_key(&opts);

    println!("");
    println!("Your key is:\t{}", key.green());
}

fn create_random_key(opts: &Opts) -> String {
    let mut rng = rand::thread_rng();

    let key = KEYS[rng.gen_range(0..KEYS.len())];

    let acc = if opts.accidentals {
        match rng.gen_range(0..3) {
            0 => FLAT,
            1 => "",
            _ => SHARP,
        }
    } else {
        ""
    };

    let quality: String = if opts.quality == Quality::Both {
        Quality::random()
    } else {
        opts.quality.clone()
    }
    .to_string();

    format!("{}{} {}", key, acc, quality)
}
