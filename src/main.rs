use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cat`
struct Args {
    /// List of files
    #[arg(value_name = "FILE", default_value="-")]
    files: Vec<String>,

    /// print the byte counts
    #[arg(short('c'), long("bytes"))]
    print_bytes: bool,

    /// print the character counts
    #[arg(short('m'), long("chars"))]
    print_chars: bool,

    /// print the newline counts
    #[arg(short('l'), long("lines"))]
    print_lines: bool,

    /// print the word counts
    #[arg(short('w'), long("words"))]
    print_words: bool,

}


fn main() {
    println!("Hello, world!");
}
