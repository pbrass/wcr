use clap::Parser;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `wc`
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

    /// print the grapheme counts
    #[arg(short('g'), long("graphemes"))]
    print_graphemes: bool,

    /// print the newline counts
    #[arg(short('l'), long("lines"))]
    print_lines: bool,

    /// print the word counts
    #[arg(short('w'), long("words"))]
    print_words: bool,

}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

//does wc count line terminators in its character counts?
//how can we tell the number of *characters* in a string

fn run(_args: Args) -> Result<()> {
    let mut totalbytes: usize = 0;
    let mut totalchars: usize = 0;
    let mut totalgraphemes: usize = 0;
    let mut totallines: usize = 0;
    let mut totalwords: usize = 0;
    for filename in _args.files {
        let mut filebytes: usize = 0;
        let mut filechars: usize = 0;
        let mut filegraphemes: usize = 0;
        let mut filewords: usize = 0;
        let mut filelines: usize = 0;
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(h_file) =>
                {
                    for line in h_file.lines() {
                        match line {
                            Ok(ln) => {
                                let linebytes = ln.len();
                                filebytes += linebytes;
                                totalbytes += linebytes;
                                let linechars = ln.chars().count();
                                filechars += linechars;
                                totalchars += linechars;
                                let linegraphemes = ln.graphemes(true).count();
                                filegraphemes += linegraphemes;
                                totalgraphemes += linegraphemes;
                                //let linewords: usize = ln.split_word_bounds().count();
                                let linewords : usize = ln.split_whitespace().count();
                                filewords += linewords;
                                totalwords += linewords;
                                filelines += 1;
                                totallines += 1;
                            },
                            _ => ()
                        }
                    }
                    print!("{filelines:>7} {filewords:>7} {filebytes:>7} {filename}\n");
                }
        }
    }
    print!("{totallines:>7} {totalwords:>7} {totalbytes:>7} total\n");
    Ok(())
}


fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
