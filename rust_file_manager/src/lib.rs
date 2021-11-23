use clap::ArgMatches;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::{
    io::Write,
    path::{Path, PathBuf},
};

pub fn run(config: &Config) -> Result<(), &'static str> {
    // 1. parse patterns
    let v_pats: Vec<Regex> = config.parse_patterns()?;

    // 2. get directories
    let v_dirs: Vec<PathBuf> = config.parse_dirs()?;

    // 3. parse optional arguments
    let mut output: Option<File> = config.parse_output();

    let size: Option<u64> = config.parse_size();

    // 4. get files and output
    let mut matched_files = Vec::with_capacity(v_dirs.len());
    for dir in v_dirs.iter() {
        get_matched_files(&mut matched_files, dir, &v_pats[..], size);

        // print or write
        if let Some(sv) = display(&matched_files, &mut output) {
            for s in sv {
                println!("{}", s);
            }
        };
        matched_files.clear();
    }

    Ok(())
}

// TODO: move this code to the outside...
pub struct Config<'a> {
    pub dirs: Vec<&'a str>,
    pub patterns: Vec<&'a str>,
    pub output: Option<&'a str>,
    pub size: Option<&'a str>,
}

impl<'a> Config<'a> {
    // you need to use explit lifetime here as well
    pub fn from_args(args: &'a ArgMatches) -> Self {
        let patterns: Vec<&'a str> = args.values_of("patterns").unwrap().collect();
        let dirs: Vec<&'a str> = args.values_of("dirs").unwrap().collect();
        let output: Option<&'a str> = args.value_of("output");
        let size: Option<&'a str> = args.value_of("size");

        Config {
            patterns,
            dirs,
            output,
            size,
        }
    }

    pub fn parse_patterns(&self) -> Result<Vec<Regex>, &'static str> {
        let mut res: Vec<Regex> = Vec::new();
        let mut parsed = false;
        for p in &self.patterns {
            if let Ok(rgx) = Regex::new(p) {
                res.push(rgx);
                parsed = true;
            } else {
                eprintln!("{} is not a valid regular expression, ignoring", p);
            }
        }
        if parsed {
            Ok(res)
        } else {
            Err("No valid regex given")
        }
    }

    pub fn parse_dirs(&self) -> Result<Vec<PathBuf>, &'static str> {
        let mut res: Vec<PathBuf> = Vec::new();
        let mut parsed = false;
        for d in &self.dirs {
            let dir = PathBuf::from(d);
            if dir.is_dir() {
                parsed = true;
                res.push(dir);
            } else {
                eprintln!("{} is an invalid directory or is inaccessible", d);
            }
        }
        if parsed {
            Ok(res)
        } else {
            Err("No valid directories given")
        }
    }

    pub fn parse_output(&self) -> Option<File> {
        let output = self.output;
        if let Some(f) = output {
            if let Ok(file) = File::create(f) {
                return Some(file);
            } else {
                eprintln!("Couldn't open {} for writing, not writing to file", f);
            }
        }
        None
    }

    pub fn parse_size(&self) -> Option<u64> {
        let size = self.size;
        if let Some(num) = size {
            if let Ok(s) = num.parse::<u64>() {
                if s > 0 {
                    return Some(s);
                }
            }
        }
        None
    }
}

pub fn get_matched_files(files: &mut Vec<MyFile>, dir: &Path, pats: &[Regex], size: Option<u64>) {
    // call get_matched_files() in itself if the given directory `dir` contains a sub-directory
    if let Ok(readdir) = fs::read_dir(dir) {
        for entry in readdir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                get_matched_files(files, &path, pats, size);
            } else if path.is_file() {
                if let Ok(file) = MyFile::from_path(&path) {
                    if size.is_none() || file.size_bytes > size.unwrap() {
                        for rgx in pats {
                            if rgx.is_match(&file.path) {
                                files.push(file);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

// represents found files
pub struct MyFile {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
}
impl MyFile {
    /// Instantiate a MyFile struct from the path of a file.
    pub fn from_path(path: &Path) -> Result<Self, &'static str> {
        let name = path
            .file_name()
            .ok_or("Couldn't parse filename")?
            .to_str()
            .ok_or("Failed conversion from osstr to str")?
            .to_string();
        let pathstr = path
            .to_str()
            .ok_or("Could not convert path to str")?
            .to_string();
        let meta = path.metadata();
        let size_bytes = match meta {
            Ok(m) => m.len(),
            Err(_) => return Err("Could not fetch metadata"),
        };
        let myfile = MyFile {
            name,
            path: pathstr,
            size_bytes,
        };
        Ok(myfile)
    }
}

pub fn display(files: &[MyFile], output: &mut Option<File>) -> Option<Vec<String>> {
    let res: Vec<String> = files.iter().map(|f| f.path.to_string()).collect();
    if let Some(outfile) = output {
        for s in res {
            writeln!(outfile, "{}", s).expect("Unable to write to file");
        }
        None
    } else {
        Some(res)
    }
}