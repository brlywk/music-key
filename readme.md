# Music Key

A tiny tool that generates a random musical key.

## Installation

Make sure Rust is installed, then run:
```bash
cargo install --git https://www.github.com/brlywk/music-key
```

## Usage

```bash
muskey [Options]

Options:
  -a, --accidentals whether accidentals should be used
  -q, --quality     the key quality to use: major, minor, both (default)
  --help            display usage information
```
Running `muskey` by itself will create a random minor or major key without accidentals. 

### Examples:
```bash
muskey              # Random major or minor key without accidentals
muskey -a           # Random major or minor key with accidentals
muskey -q M         # Random major key without accidentals
muskey -q min -a    # Random minor key with accidentals
```

## Motivation

Because I love making music but tend to always use the keys I'm most comfortable with (looking at you, C Minor).
As I also wanted to learn Rust, I thought the very limited scope of this little tool would be a perfect "first project".

## License

I don't know, but you if you know Rust you could probably write this thing yourself in the same time it took me to write
this sentence.
