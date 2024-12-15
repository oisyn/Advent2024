use crate::*;
use anyhow::{bail, Result};
use memmap::Mmap;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

pub struct Input(Mmap, std::time::Instant, bool);

impl Input {
    pub fn lines(&self) -> Lines {
        Lines(self.bytes())
    }

    pub fn paragraphs(&self) -> Paragraphs {
        Paragraphs(self.bytes())
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn str(&self) -> &str {
        to_str(&self.0)
    }

    pub fn is_example(&self) -> bool {
        self.2
    }
}

impl AsRef<[u8]> for Input {
    fn as_ref(&self) -> &[u8] {
        self.bytes()
    }
}

impl AsRef<str> for Input {
    fn as_ref(&self) -> &str {
        to_str(self.bytes())
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        let n = std::time::Instant::now();
        let d = n - self.1;
        if self.2 {
            println!("\x1b[91mEXAMPLE INPUT\x1b[0m");
        }
        println!("Time spent: {:.1}µs", d.as_nanos() as f32 / 1000.0);
    }
}

pub struct Lines<'a>(&'a [u8]);

impl<'a> Iterator for Lines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        let start = self.0;
        let mut end = self.0.len();
        let mut next = self.0.len();
        for i in 0..self.0.len() {
            let c = self.0[i];
            if c == b'\r' || c == b'\n' {
                end = i;
                next = i + 1;
                if c == b'\r' && self.0.len() > next && self.0[next] == b'\n' {
                    next += 1;
                }
                break;
            }
        }
        self.0 = &self.0[next..];
        Some(to_str(&start[..end]))
    }
}

pub struct Paragraphs<'a>(&'a [u8]);

impl<'a> Iterator for Paragraphs<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let start_pos = self.0.iter().position(|&c| !is_nl(c))?;
        let start = &self.0[start_pos..];
        let mut cur_pos = 1;
        loop {
            let Some(mut end_pos) = start[cur_pos..].iter().position(|&c| is_nl(c)) else {
                self.0 = &[];
                return Some(to_str(start));
            };

            end_pos += cur_pos;

            if start[end_pos] == b'\r' {
                end_pos += 2;
            } else {
                end_pos += 1;
            }

            if end_pos == start.len() || is_nl(start[end_pos]) {
                self.0 = &start[end_pos..];
                return Some(to_str(&start[..end_pos]));
            }

            cur_pos = end_pos + 1;
        }
    }
}

fn get_input_path(day: &str) -> Result<(PathBuf, bool)> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        Ok((Path::new(day).join("data/input.txt"), false))
    } else if args.len() == 2 && args[1].starts_with("-e") {
        Ok((
            Path::new(day).join(format!("data/example{}.txt", &args[1][2..])),
            true,
        ))
    } else if args.len() == 3 && args[1] == "-i" {
        Ok((PathBuf::from(&args[2]), false))
    } else {
        bail!("Bad command line arguments. Expected nothing, `-e`, or `-i <file>`")
    }
}

pub fn open_input(day: &str) -> Result<Input> {
    let t = std::time::Instant::now();
    let (path, is_example) = get_input_path(day)?;
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };

    #[cfg(feature = "validation")]
    if !std::str::from_utf8(&mmap)?.is_ascii() {
        bail!("Input contains non-ascii data");
    }

    Ok(Input(mmap, t, is_example))
}
