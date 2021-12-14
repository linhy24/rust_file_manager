use clap::ArgMatches;
use regex::Regex;
use shlex;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::{
    io::Write,
    path::{Path, PathBuf},
};

/**************************** rust_add starts **************************** */
pub fn run_add(config: &AddConfig) -> Result<(), &'static str> {
    let v_dirs: Vec<PathBuf> = config.parse_dirs()?;
    let v_files: Vec<&str> = config.parse_files()?;

    for d in v_dirs {
        for f in &v_files {
            let file_path = d.join(f);
            match std::fs::write(file_path, "") {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Failed to add file {}: {}", f, err);
                }
            }
        }
    }

    Ok(())
}

pub struct AddConfig<'a> {
    pub dirs: Vec<&'a str>,
    pub files: Vec<&'a str>,
}

impl<'a> AddConfig<'a> {
    pub fn from_args(args: &'a ArgMatches) -> Self {
        let dirs: Vec<&'a str> = args.values_of("dirs").unwrap().collect();
        let files: Vec<&'a str> = args.values_of("files").unwrap().collect();

        AddConfig { dirs, files }
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

    pub fn parse_files(&self) -> Result<Vec<&str>, &'static str> {
        let mut res: Vec<&str> = Vec::new();

        for f in &self.files {
            res.push(*f);
        }

        Ok(res)
    }
}

/**************************** rust_add ends **************************** */

/**************************** rust_remove starts **************************** */
pub struct RemoveConfig<'a> {
    pub dirs: Vec<&'a str>,
    pub files: Vec<&'a str>,
}

pub fn run_remove(config: &RemoveConfig) -> Result<(), &'static str> {
    let v_dirs: Vec<PathBuf> = config.parse_dirs()?;
    let v_files: Vec<&str> = config.parse_files()?;

    for d in v_dirs {
        for f in &v_files {
            let file_path = d.join(f);
            match std::fs::remove_file(file_path) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Failed to remove file {}: {}", f, err);
                }
            }
        }
    }

    Ok(())
}

impl<'a> RemoveConfig<'a> {
    pub fn from_args(args: &'a ArgMatches) -> Self {
        let dirs: Vec<&'a str> = args.values_of("dirs").unwrap().collect();
        let files: Vec<&'a str> = args.values_of("files").unwrap().collect();

        RemoveConfig { dirs, files }
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

    pub fn parse_files(&self) -> Result<Vec<&str>, &'static str> {
        let mut res: Vec<&str> = Vec::new();

        for f in &self.files {
            res.push(*f);
        }

        Ok(res)
    }
}

/**************************** rust_remove ends **************************** */

/**************************** rust_find starts **************************** */
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

        if let Some(sv) = display(&matched_files, &mut output) {
            // string implements clone
            let paths = matched_files
                .iter()
                .map(|x| x.path.clone())
                .collect::<Vec<_>>();
            if let Some(exec) = config.exec {
                // split strings in accordance with shell expansion
                let cmd = shlex::split(exec).unwrap();
                // find the index of the replace string
                // the replace string technically does not have to be there
                let pos = cmd
                    .iter()
                    .position(|x| x == config.replace.unwrap())
                    .unwrap();
                if config.all {
                    let cmd = [&cmd[..pos], &paths, &cmd[(pos + 1)..]].concat();
                    // run command with all files as args
                    Command::new(&cmd[0])
                        .args(&cmd[1..])
                        .spawn()
                        .expect("failed to run process");
                } else {
                    // run 1 command per each file
                    // TODO: specify threads
                    for path in paths {
                        let cmd = [&cmd[..pos], &[path], &cmd[(pos + 1)..]].concat();
                        Command::new(&cmd[0])
                            .args(&cmd[1..])
                            .spawn()
                            .expect("failed to run process");
                    }
                }
            } else {
                for s in sv {
                    println!("{}", s);
                }
            }
        };
        matched_files.clear();
    }
    Ok(())
}

// TODO: move this code to the outside...
pub struct FindConfig<'a> {
    pub dirs: Vec<&'a str>,
    pub patterns: Vec<&'a str>,
    pub output: Option<&'a str>,
    pub size: Option<&'a str>,
    pub exec: Option<&'a str>,
    pub replace: Option<&'a str>,
    pub all: bool,
}

impl<'a> FindConfig<'a> {
    // you need to use explit lifetime here as well
    pub fn from_args(args: &'a ArgMatches) -> Self {
        let patterns: Vec<&'a str> = args.values_of("patterns").unwrap().collect();
        let dirs: Vec<&'a str> = args.values_of("dirs").unwrap().collect();
        let output: Option<&'a str> = args.value_of("output");
        let size: Option<&'a str> = args.value_of("size");
        let exec: Option<&'a str> = args.value_of("exec");
        let replace: Option<&'a str> = args.value_of("replace");
        let all: bool = args.is_present("all");

        FindConfig {
            patterns,
            dirs,
            output,
            size,
            exec,
            replace,
            all,
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
    pub fn parse_exec(&self) {
        // should return a vector of args to pass
    }
    pub fn parse_replace(&self) {
        // should check that the string is valid
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

/**************************** rust_find ends **************************** */

/**************************** rust_tr starts **************************** */
pub struct TrConfig<'a> {
    pub path: Option<&'a str>,
    pub file: Option<&'a str>,
    pub delete: Option<&'a str>,
    pub replace: Vec<&'a str>,
    pub simulate: bool,
}

pub fn run_tr(config: &TrConfig) -> Result<(), &'static str> {
    let mut content: Option<String> = config.parse_file_path();
    let delete: Option<&str> = config.delete;
    let _file: Option<File> = config.parse_file();
    let path: Option<PathBuf> = config.parse_path();
    let v_replace: Option<Vec<&str>> = config.parse_replace();

    // call aux functions for implementing delete & replace
    if delete.is_some() {
        content = delete_words(&mut content, delete);
    }

    if let Some(r) = v_replace {
        content = replace_words(&mut content, r);
    }

    if let Some(c) = content {
        if config.simulate {
            println!("{}", c);
        } else {
            if let Some(p) = path {
                // writeln!(file.unwrap(), "{}", c).expect("Unable to write to file");
                if let Some(f) = config.file {
                    let file_path = p.join(f);
                    match fs::write(file_path, c) {
                        Ok(_) => {
                            println!("Your operation is successful this time!")
                        }
                        Err(err) => {
                            eprintln!("Failed to write to file, {}", err);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

impl<'a> TrConfig<'a> {
    pub fn from_args(args: &'a ArgMatches) -> Self {
        let path: Option<&'a str> = args.value_of("path");
        let file: Option<&'a str> = args.value_of("file");
        let delete: Option<&'a str> = args.value_of("delete");
        let simulate: bool = args.is_present("simulate");
        let mut replace: Vec<&'a str> = Vec::new();
        if let Some(val) = args.values_of("replace") {
            replace = val.collect();
        }

        TrConfig {
            path,
            file,
            delete,
            replace,
            simulate,
        }
    }

    pub fn parse_file(&self) -> Option<File> {
        let mut res: Option<File> = None;

        if let Some(p) = self.parse_path() {
            if let Some(f) = self.file {
                let file_path = p.join(f);
                match File::open(file_path) {
                    Ok(r) => {
                        res = Some(r);
                    }
                    Err(err) => {
                        eprintln!("Failed to open file {}: {}", f, err);
                    }
                }
            }
        }

        res
    }

    pub fn parse_replace(&self) -> Option<Vec<&str>> {
        let mut res = Vec::new();
        let mut i = 0;
        if self.replace.len() == 2 {
            while i < 2 {
                res.push(self.replace[i]);
                i += 1;
            }
            return Some(res);
        }
        None
    }

    pub fn parse_path(&self) -> Option<PathBuf> {
        let mut res = None;
        if let Some(p) = self.path {
            let path = PathBuf::from(p);
            if path.is_dir() {
                res = Some(path);
            } else {
                eprintln!("{} is an invalid directory or is inaccessible", p);
            }
        }
        res
    }

    // need to address borrowing issues in this function
    pub fn parse_file_path(&self) -> Option<String> {
        let mut res: Option<String> = None;

        if let Some(p) = self.parse_path() {
            if let Some(f) = self.file {
                let file_path = p.join(f);
                match fs::read_to_string(file_path) {
                    Ok(r) => {
                        res = Some(r);
                    }
                    Err(err) => {
                        eprintln!("Failed to read file {}: {}", f, err);
                    }
                }
            }
        }

        res
    }
}

pub fn delete_words(content: &mut Option<String>, delete: Option<&str>) -> Option<String> {
    let mut res = String::from("");
    if let Some(c) = content {
        if let Some(d) = delete {
            res = c.replace(d, "");
        }
    }

    Some(res)
}

pub fn replace_words(content: &mut Option<String>, replace: Vec<&str>) -> Option<String> {
    let mut res = String::from("");
    if let Some(c) = content {
        res = c.replace(replace[0], replace[1]);
    }
    Some(res)
}

/**************************** rust_tr ends **************************** */

/**************************** rust_grep start *****************************/

pub struct GrepConfig<'a> {
    pub patterns: Vec<&'a str>,
    pub filenames: Vec<&'a str>,
}

pub fn run_grep(config: &GrepConfig) -> Result<(), &'static str> {
    let v_patterns: Vec<Regex> = config.parse_patterns()?;
    let v_files: Vec<&str> = config.parse_files()?;

    for pattern in v_patterns {
        println!("Searching for lines matching {}", pattern);
        for filename in &v_files {
            let mut f = File::open(filename).unwrap();
            let mut contents = String::new();

            f.read_to_string(&mut contents);
            let mut line_number = 1;
            println!("Looking inside {}", filename);
            for line in contents.lines() {
                // if line.contains(pattern) {
                if pattern.is_match(line) {
                    println!("{}: {}", line_number, line);
                }
                line_number += 1;
            }
            println!("Done looking inside {}", filename);
        }
        println!("Done searching for lines matching {}", pattern);
    }

    Ok(())
}

impl<'a> GrepConfig<'a> {
    pub fn from_args(args: &'a ArgMatches) -> Self {
        let patterns: Vec<&'a str> = args.values_of("patterns").unwrap().collect();
        let filenames: Vec<&'a str> = args.values_of("filenames").unwrap().collect();

        GrepConfig {
            patterns,
            filenames,
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

    pub fn parse_files(&self) -> Result<Vec<&str>, &'static str> {
        let mut res: Vec<&str> = Vec::new();

        for f in &self.filenames {
            res.push(*f);
        }

        Ok(res)
    }
}
/**************************** rust_grep ends *****************************/
