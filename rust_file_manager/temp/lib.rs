use clap::ArgMatches;
use regex::Regex;
use std::fs::File;
use std::{
    io::Write,
    path::{Path, PathBuf},
};

pub fn run_add(config: &AddConfig) -> Result<(), &'static str> {

    let v_dirs: Vec<PathBuf> = config.parse_dirs()?;

    let mut output: Option<File> = config.parse_filename();

    Ok(())
}

pub fn run_find(config: &FindConfig) -> Result<(), &'static str> {
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

pub struct FindConfig<'a> {
    pub dirs: Vec<&'a str>,
    pub patterns: Vec<&'a str>,
    pub output: Option<&'a str>,
    pub size: Option<&'a str>,
}

pub struct AddConfig<'a> {
    pub dirs: Vec<&'a str>,
    pub filename: Option<&'a str>
}

// parse command-line arguments into struct Config
impl<'a> FindConfig<'a> {
    pub fn from_args(args: &'a ArgMatches) -> Self {
        FindConfig {
            dirs: args.values_of("dirs").unwrap().collect(),
            patterns: args.values_of("patterns").unwrap().collect(),
            output: args.value_of("output"),
            size: args.value_of("size"),
        }
    }
}

impl<'a> AddConfig<'a> {
    pub fn from_args(args: &'a ArgMatches) -> Self {
        AddConfig {
            dirs: args.values_of("dirs").unwrap().collect(),
            filename: Some(args.value_of("filename")).unwrap(),
        }
    }
}

impl<'a> AddConfig<'a> {
    pub fn parse_dirs(&self) -> Result<Vec<PathBuf>, &'static str> {
        let mut res = Vec::new();

        for ele in &self.dirs {
            match PathBuf::from(ele).metadata() {
                Ok(_) => res.push(PathBuf::from(ele)),
                Err(err) => eprintln!("Invalid path error: {}", err),
            }
        }

        if res.is_empty() {
            return Err("All paths are invalid!");
        }
        Ok(res)
    }

    pub fn parse_filename(&self) -> Option<File> {
        match self.filename {
            Some(name) => match File::create(name) {
                Ok(file) => Some(file),
                Err(err) => {
                    eprintln!("Invalid file path error: {}", err);
                    None
                }
            },
            None => None,
        }
    }
}

impl<'a> FindConfig<'a> {
    pub fn parse_patterns(&self) -> Result<Vec<Regex>, &'static str> {
        let mut res = Vec::new();

        for ele in &self.patterns {
            match Regex::new(ele) {
                Ok(re) => res.push(re),
                Err(err) => eprintln!("Invalid pattern error: {}", err),
            }
        }

        // if none of patterns match, return Err()
        if res.is_empty() {
            return Err("All patterns are invalid!");
        }
        Ok(res)
    }

    pub fn parse_dirs(&self) -> Result<Vec<PathBuf>, &'static str> {
        let mut res = Vec::new();

        for ele in &self.dirs {
            match PathBuf::from(ele).metadata() {
                Ok(_) => res.push(PathBuf::from(ele)),
                Err(err) => eprintln!("Invalid path error: {}", err),
            }
        }

        if res.is_empty() {
            return Err("All paths are invalid!");
        }
        Ok(res)
    }

    pub fn parse_output(&self) -> Option<File> {
        match self.output {
            Some(output) => match File::create(output) {
                Ok(file) => Some(file),
                Err(err) => {
                    eprintln!("Invalid file path error: {}", err);
                    None
                }
            },
            None => None,
        }
    }

    pub fn parse_size(&self) -> Option<u64> {
        match self.size {
            Some(size) => match size.parse::<u64>() {
                Ok(res) => Some(res),
                Err(err) => {
                    eprintln!("Invalid size error: {}", err);
                    None
                }
            },
            None => None,
        }
    }
}

pub struct MyFile {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
}

impl MyFile {
    /// Instantiate a MyFile struct from the path of a file.
    pub fn from_path(path: &Path) -> Result<Self, &'static str> {
        // possibly error when unwrap()
        let mut err = false;
        if path.file_name().is_none() {
            eprintln!("file name invalid");
            err = true;
        } else if path.to_str().is_none() {
            eprintln!("file path invalid");
            err = true;
        } else if path.metadata().is_err() {
            eprintln!("file size parse error");
            err = true;
        }

        if err {
            Err("MyFile struct creation error")
        } else {
            Ok(MyFile {
                name: String::from(path.file_name().unwrap().to_str().unwrap()),
                path: String::from(path.to_str().unwrap()),
                size_bytes: path.metadata().unwrap().len(),
            })
        }
    }
}

pub fn get_matched_files(files: &mut Vec<MyFile>, dir: &Path, pats: &[Regex], size: Option<u64>) {
    // call get_matched_files() in itself if the given directory `dir` contains a sub-directory

    if std::fs::read_dir(dir).is_err() {
        eprintln!("Invalid directory!");
    } else {
        for rd in std::fs::read_dir(dir).unwrap() {
            // To get path from read_dir() result
            let path = rd.unwrap().path();

            // call get_matched_files on path that is_dir (convert path from PathBuf type to Path type)
            // else, if path is not a directory, that means get_matched_files have already found the matched file,
            // then you should get the last element (call file_name) of path and match it with Regex

            // add newly constructed file to files vector
            if path.is_dir() {
                get_matched_files(files, Path::new(path.to_str().unwrap()), pats, size);
            } else {
                // println!("{}", path.file_name().unwrap().to_str().unwrap());
                for regex in pats {
                    if regex.is_match(path.file_name().unwrap().to_str().unwrap()) {
                        let mut check = true;

                        // check file size threshold if --size exists
                        if let Some(s) = size {
                            // println!("{}", path.metadata().unwrap().len());
                            check = path.metadata().unwrap().len() >= s;
                        }

                        // println!("{}", check);
                        if check {
                            if MyFile::from_path(&path).is_err() {
                                eprintln!("Invalid file path!");
                            } else {
                                // add the file struct to files vector
                                files.push(MyFile::from_path(&path).unwrap());
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn display(files: &[MyFile], output: &mut Option<File>) -> Option<Vec<String>> {
    let mut res = Vec::new();

    if output.is_none() {
        for file in files {
            res.push(file.path.clone());
        }
        Some(res)
    } else {
        for file in files {
            match writeln!(output.as_ref().unwrap(), "{}", file.path) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Invalid writeln error: {}", err);
                }
            }
        }
        None
    }
}