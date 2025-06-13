use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use unicode_segmentation::UnicodeSegmentation;
use std::ops::Add;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `wc`
struct Args {
    /// List of files
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// print the byte counts
    #[arg(short('c'), long("bytes"), conflicts_with("print_chars"), conflicts_with("print_graphemes"))]
    print_bytes: bool,

    /// print the character counts
    #[arg(short('m'), long("chars"), conflicts_with("print_bytes"), conflicts_with("print_graphemes"))]
    print_chars: bool,

    /// print the grapheme counts
    #[arg(short('g'), long("graphemes"), conflicts_with("print_chars"), conflicts_with("print_bytes"))]
    print_graphemes: bool,

    /// print the line counts
    #[arg(short('l'), long("lines"))]
    print_lines: bool,

    /// print the word counts
    #[arg(short('w'), long("words"), conflicts_with("print_unicode_words"), conflicts_with("print_unicode_bounds"))]
    print_words: bool,

    /// print the unicode word counts
    #[arg(short('u'), long("unicode_words"), conflicts_with("print_words"),conflicts_with("print_unicode_bounds"))]
    print_unicode_words: bool,

    /// print the unicode word bounds count
    #[arg(short('b'), long("unicode_bounds"), conflicts_with("print_words"),conflicts_with("print_unicode_words"))]
    print_unicode_bounds: bool,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct FileCounts {
    bytes: usize,
    chars: usize,
    graphemes: usize,
    words: usize,
    unicode_words: usize,
    unicode_bounds: usize,
    lines: usize,
}

// Making it quicker to add the file counts into the total count, I guess.
impl Add for FileCounts {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            bytes: self.bytes + rhs.bytes,
            chars: self.chars + rhs.chars,
            graphemes: self.graphemes + rhs.graphemes,
            words: self.words + rhs.words,
            unicode_words: self.unicode_words + rhs.unicode_words,
            unicode_bounds: self.unicode_bounds + rhs.unicode_bounds,
            lines: self.lines + rhs.lines,
        }
    }
}

// Get all the counts for a single buffer thingy
fn count(mut file: impl BufRead) -> Result<FileCounts> {
    let mut count = FileCounts::default();
    let mut line = String::new();
    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 { break;}
        count.lines += 1;
        count.bytes += line_bytes;
        count.graphemes += line.graphemes(true).count();
        count.chars += line.chars().count();
        count.words += line.split_whitespace().count();
        count.unicode_words += line.unicode_words().count();
        count.unicode_bounds += line.split_word_bounds().count();
        line.clear();
    }
    Ok(count)
}

fn printcounts(_args: &Args, fcounts: FileCounts, filename: String)  {
    if _args.print_lines {print!("{:>8}",fcounts.lines)}
    if _args.print_words {print!("{:>8}",fcounts.words)}
    if _args.print_unicode_words {print!("{:>8}",fcounts.unicode_words)}
    if _args.print_unicode_bounds {print!("{:>8}",fcounts.unicode_bounds)}
    if _args.print_bytes {print!("{:>8}",fcounts.bytes)}
    if _args.print_graphemes {print!("{:>8}",fcounts.graphemes)}
    if _args.print_chars {print!("{:>8}",fcounts.chars)}
    if filename != "-" {print!(" {filename}")}
    print!("\n");
}

// run the program against the arguments - loop through the filenames, open the files, and count them
fn run(mut _args: Args) -> Result<()> {
    let mut totals = FileCounts::default();
    
    // if there are no flags, then we want lines, words and bytes
    if !(_args.print_bytes || _args.print_graphemes || _args.print_chars || _args.print_words
        || _args.print_lines || _args.print_unicode_words || _args.print_unicode_bounds)
    {
        _args.print_bytes = true;
        _args.print_lines = true;
        _args.print_words = true;
    }
    let filecount = _args.files.len();
    let names = _args.files.clone();
    for filename in names {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(h_file) => {
                let fcounts = count(h_file)?;
                printcounts(&_args, fcounts, filename);
                totals = totals + fcounts;
            }
        }
    }
    if filecount > 1 { printcounts(&_args, totals, String::from("total")); }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
