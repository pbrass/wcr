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


#[derive(Debug, Copy, Clone, PartialEq)]
struct FileCounts {
    bytes: usize,
    chars: usize,
    graphemes: usize,
    words: usize,
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
            lines: self.lines + rhs.lines,
        }
    }
}

// Get all the counts for a single buffer thingy
fn count(mut file: impl BufRead) -> Result<FileCounts> {
    let mut count = FileCounts{ bytes: 0, chars: 0, graphemes: 0, words: 0, lines: 0 };
    let mut line = String::new();
    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 { break;}
        count.lines += 1;
        count.bytes += line_bytes;
        count.graphemes += line.graphemes(true).count();
        count.chars = line.chars().count();
        count.words = line.split_whitespace().count();
    }
    Ok(count)
}

// run the program against the arguments - loop through the filenames, open the files, and count them
fn run(mut _args: Args) -> Result<()> {
    let mut totals = FileCounts{bytes: 0, chars: 0, graphemes: 0, words: 0, lines: 0 };
    
    // if there are no flags, then we want lines, words and bytes
    if !(_args.print_bytes || _args.print_graphemes || _args.print_chars || _args.print_words || _args.print_lines)
    {
        _args.print_bytes = true;
        _args.print_lines = true;
        _args.print_words = true;
    }
    let filecount = _args.files.len();
    for filename in _args.files {
        //stats for the current file

        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(h_file) => {
                let fcounts = count(h_file)?;
                totals = totals + fcounts;
                /*
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
                
                 */
                
                if _args.print_lines {print!("{:>8}",fcounts.lines)}
                if _args.print_words {print!("{:>8}",fcounts.words)}
                if _args.print_bytes {print!("{:>8}",fcounts.bytes)}
                if _args.print_graphemes {print!("{:>8}",fcounts.graphemes)}
                if _args.print_chars {print!("{:>8}",fcounts.chars)}
                if filename != "-" {print!(" {filename}")}
                print!("\n");
            }
        }
    }
    if filecount > 1 {
        if _args.print_lines {print!("{:>8}",totals.lines)}
        if _args.print_words {print!("{:>8}",totals.words)}
        if _args.print_bytes {print!("{:>8}",totals.bytes)}
        if _args.print_graphemes {print!("{:>8}",totals.graphemes)}
        if _args.print_chars {print!("{:>8}",totals.chars)}
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
