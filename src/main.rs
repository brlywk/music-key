mod opts;
use colored::Colorize;
use opts::{Opts, Quality};
use rand::Rng;

// We really don't need to over-engineer this and dependency inject the
// following values into create_random_key ;)

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

// Just writing a few tests here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_random_key_basic() {
        let opts = Opts {
            accidentals: false,
            quality: Quality::Both,
        };
        let key = create_random_key(&opts);

        assert!(KEYS.iter().any(|&k| key.starts_with(k)));
        assert!(key.ends_with("Major") || key.ends_with("Minor"));
        assert!(!key.contains("#") && !key.contains("b"));
    }

    #[test]
    fn test_create_random_key_with_accidentals() {
        let opts = Opts {
            accidentals: true,
            quality: Quality::Both,
        };
        let key = create_random_key(&opts);

        // accidentals might be there, or not... who knows?
        assert!(KEYS.iter().any(|&k| key.starts_with(k)));
        assert!(key.ends_with("Major") || key.ends_with("Minor"));
    }

    #[test]
    fn test_create_random_key_specific_quality() {
        let opts = Opts {
            accidentals: false,
            quality: Quality::Major,
        };
        let key = create_random_key(&opts);

        assert!(KEYS.iter().any(|&k| key.starts_with(k)));
        assert!(key.ends_with("Major"));
    }
}
