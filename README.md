# Rust_file_manager

## Team members
<strong>Yonas Mekonnen</strong><br/><strong>Hongyang Lin</strong><br/><strong>Spencer Chan</strong>

## Introduction and background
Our project is to make a CLI that will perform basic linux file management. The CLI would have 6 commands (and additional flags) to emulate basic linux commands.

These commands would map to the linux commands listed below.
- rust-add -> touch and mkdir
- rust-remove  -> rm
- rust-find -> rust_find project
- rust-grep -> grep
- rust-xargs -> xargs
- rust-diff -> diff
- rust-tr -> (tr++) modified <strong>tr</strong> for word substitution
- -thread -> make use of threading

In the end, users should be able to call a “super” command that unifies the functionalities of all three commands above. For instance, a command can contain the functionalities of -a(add), -r(remove), and -f(find). We expect to make use of rust crates like clap to implement our project.

We plan to adopt the idea of threading as our stretch goal. The idea is that some commands may have multiple inputs so we can use threads to process each input. Users use a flag -thread to indicate that they want to run the command using threads.

## Example usage
- <strong>find</strong>: target/debug/rust find -p '.*\.rs' -d ./src
- <strong>add</strong>: target/debug/rust add -f 'test.txt' -d ./src ./tests
- <strong>remove</strong>: target/debug/rust remove -f 'test.txt' -d ./src ./tests
- <strong>tr++</strong>: target/debug/rust tr -f 'test.txt' -p ./src -d "Hey" (Please add your own testing content into 'test.txt')

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
