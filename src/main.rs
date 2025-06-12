use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::fs;
use std::io::{self, BufRead, BufReader};
use unicode_segmentation::UnicodeSegmentation;

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
//it does in byte-count mode
//how can we tell the number of *characters* in a string

fn run(_args: Args) -> Result<()> {
    let mut totalbytes: usize = 0;
    let mut totalchars: usize = 0;
    let mut totalgraphemes: usize = 0;
    let mut totallines: usize = 0;
    let mut totalwords: usize = 0;
    let mut pbytes: bool = _args.print_bytes;
    let pgraphemes: bool = _args.print_graphemes;
    let mut pwords: bool = _args.print_words;
    let mut plines: bool = _args.print_lines;
    let pchars: bool = _args.print_chars;
    // if there are no flags, then we want lines, words and bytes
    if !(_args.print_bytes || _args.print_graphemes || _args.print_chars || _args.print_words || _args.print_lines)
    {
        pbytes = true;
        plines = true;
        pwords = true;
    }
    let filecount = _args.files.len();
    for filename in _args.files {
        //stats for the current file
        let mut filebytes: usize = 0;
        let mut filechars: usize = 0;
        let mut filegraphemes: usize = 0;
        let mut filewords: usize = 0;
        let mut filelines: usize = 0;
        
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(h_file) => {
                for line in h_file.lines() {
                    match line {
                        Ok(ln) => {
                            let linebytes = ln.len();
                            filebytes += linebytes;
                            // for some reason we need to add two characters to the bytecount for 
                            // each line, if we are reading from stdin, to get the same result
                            // as GNU wc
                            if filename == "-" {filebytes += 2}
                            
                            // for some reason, ending newlines are part of the "character" count
                            // in GNU wc? So we have to add one to the character count
                            let linechars = ln.chars().count() + 1; 
                            filechars += linechars;
                            totalchars += linechars;
                            
                            // I'm not going to count the newline as a grapheme. Do you think
                            // I should? Doesn't seem like one to me.
                            let linegraphemes = ln.graphemes(true).count();
                            filegraphemes += linegraphemes;
                            totalgraphemes += linegraphemes;
                            
                            // The truly unicode way to count words would be to split on word bounds
                            // but this includes basically all non-alpha characters, and what
                            // we mostly mean by words is just whitespace-delimited, so we won't
                            // use the unicode "split_word_bounds" for now, but I leave it in commented
                            // as something to think about, maybe an option in the future.
                            //let linewords: usize = ln.split_word_bounds().count();
                            let linewords: usize = ln.split_whitespace().count();
                            filewords += linewords;
                            totalwords += linewords;
                            
                            
                            filelines += 1;
                            totallines += 1;
                        }
                        _ => (/* Should we do something here? Log an error?*/),
                    }
                }
                
                // so if we're not reading from stdin, rather than using the number of bytes
                // in each line, we can just get the file metadata 
                if filename != "-" {
                    let fname = filename.clone(); // thanks, borrow-checker
                    filebytes = fs::metadata(fname)?.len() as usize;
                    totalbytes += filebytes;
                }
                if plines {print!("{filelines:>8}")}
                if pwords {print!("{filewords:>8}")}
                if pbytes {print!("{filebytes:>8}")}
                if pgraphemes {print!("{filegraphemes:>8}")}
                if pchars {print!("{filechars:>8}")}
                if filename != "-" {print!(" {filename}")}
                print!("\n");
            }
        }
    }
    if filecount > 1 {
        if plines {print!("{totallines:>8}")}
        if pwords {print!("{totalwords:>8}")}
        if pbytes {print!("{totalbytes:>8}")}
        if pgraphemes {print!("{totalgraphemes:>8}")}
        if pchars {print!("{totalchars:>8}")}
        print!(" total\n");
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
