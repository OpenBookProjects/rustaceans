//use anyhow::Error;

use std::io::BufRead;
use std::io::BufReader;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug)]
pub struct Flags{
    show_linenum: bool,
    show_nonempty_files: bool,
    case_insensitive: bool,
    invert_match: bool,
    exact_match: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        /* unimplemented!(
            "Given the flags {flags:?} implement your own 'Flags' struct to handle flags-related logic"
        ); */
        Flags  {
            show_linenum: flags.contains(&"-n"),
            show_nonempty_files: flags.contains(&"-l"),
            case_insensitive: flags.contains(&"-i"),
            invert_match: flags.contains(&"-v"),
            exact_match: flags.contains(&"-x"),
        }
    }
}

//pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> anyhow::Result<Vec<String>> {
    /* unimplemented!(
        "Search the files '{files:?}' for '{pattern}' pattern and save the matches in a vector. Your search logic should be aware of the given flags '{flags:?}'"
    ); */
    let mut result: Vec<String> = Vec::new();
    for file in files {
        let reader = BufReader::new(std::fs::File::open(file)?);
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let cmp = |a:&str,b:&str|{
                if flags.exact_match {
                    a == b
                } else {
                    a.contains(b)
                }
            };
            
            let matched = if flags.case_insensitive {
                cmp(line.to_lowercase().as_str(), 
                    pattern.to_lowercase().as_str())
            } else {
                cmp(line.as_str(), pattern)
            };

            if matched ^ flags.invert_match {
                if flags.show_nonempty_files {
                    result.push(file.to_string());
                    break;
                } else {
                    let mut out = String::new();
                    if files.len() > 1 {
                        out += &format!("{}:", file);
                    }
                    if flags.show_linenum {
                        out += &format!("{}:", i+1);
                    }
                    out += &line;
                    result.push(out);
                }
            }
        }
    }
    Ok(result)
}





