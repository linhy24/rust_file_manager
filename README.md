# Rust_file_manager

## Team members
<strong>Yonas Mekonnen</strong><br/><strong>Hongyang Lin</strong><br/><strong>Spencer Chan</strong>

## Introduction and background
Our project is to make a CLI that will perform basic linux file management. The CLI would have 6 commands (and additional flags) to emulate basic linux commands.

These commands would map to the linux commands listed below.
- rust-add -> touch
- rust-remove  -> rm
- rust-find -> rust_find, extended with the --exec flag to mimic find --exec
- rust-grep -> grep
- rust-diff -> diff
- rust-tr -> (tr++) modified <strong>tr</strong> for word deletion & substitution

We plan to adopt the idea of threading as our stretch goal. The idea is that some commands may have multiple inputs so we can use threads to process each input. Users use a flag -thread to indicate that they want to run the command using threads.

## Example usage
- <strong>find</strong>: `target/debug/rust find -p '.*\.rs' -d ./src`
    - use find with the --exec flag to run a command once per found file, optionally with the --all flag to run a command with all files
    - `target/debug/rust find -p '.*\.rs' -d ./src --exec 'echo {}' --replace {}`
    - `target/debug/rust find -p '.*\.rs' -d ./src --exec 'sort {}' --replace {} --all`
- <strong>add</strong>: `target/debug/rust add -f 'test.txt' -d ./src ./tests`
- <strong>remove</strong>: `target/debug/rust remove -f 'test.txt' -d ./src ./tests`
- <strong>tr++</strong>: 
    - `target/debug/rust tr -f 'test.txt' -p ./src -d "Hey"`
    - `target/debug/rust tr -f 'test.txt' -p ./src -r "Hey" "Hi"`(Please add your own testing content into 'test.txt')
    - use tr without actually modify the file (only simulate the result after replacing/deleting):
    - `target/debug/rust tr -f 'test.txt' -p ./src -r "Hey" "Hi" --simulate`
    
- <strong>grep</strong>: `target/debug/rust grep --patterns '^\[' --filenames 'Cargo.toml'`

## Goals
- 100% goal, representing what you expect to achieve:<br/>
Get all 6 commands/flags to work

- 75% goal (if things go slower than expected):<br/>
Get 4 out of 6 commands/flags to work

- 125% goal (if the project turns out to be easier than you thought):<br/>
Implement threading for each command and add more flags/options for customization

## Specific aims and objectives
- Emulate the functionality of linux commands using Rust
- Unify the commands under one “super” command/CLI

## Cited references
https://github.com/cmsc388z/assignments/blob/main/assignment3/assignment3.md<br/> 
https://docs.rs/clap/2.33.3/clap/
