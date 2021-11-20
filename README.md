# Rust_file_manager

## Team members
#### Yonas Mekonnen<br/>Hongyang Lin<br/>Spencer Chan

## Introduction and background
Our project is to make a CLI that will perform basic linux file management. The CLI would have 3 commands for adding, finding, and removing files and directories. 

These commands would map to the linux commands listed below.
- rust-add -> touch and mkdir
- rust-remove  -> rm
- rust-find -> rust_find project
- rust-grep -> grep
- rust-xargs -> xargs
- -thread -> make use of threading

For each command, we plan to adopt the idea of threading. Some commands may have multiple inputs so we can use threads to process each input. Users use a flag -thread to indicate that they want to run the command using threads.

In the end, users should be able to call a “super” command that unifies the functionalities of all three commands above, specified by -a(add), -r(remove), and -f(find). We expect to make use of rust crates like clap to implement our project.

## Goals
- 100% goal, representing what you expect to achieve:<br/>
Get all 6 commands/flags to work

- 75% goal (if things go slower than expected):<br/>
Get 4 out of 6 commands/flags to work

- 125% goal (if the project turns out to be easier than you thought):<br/>
Add more flags/options for customization

## Specific aims and objectives
- Emulate the functionality of linux commands using Rust
- Unify the commands under one “super” command/CLI

## Cited references
https://github.com/cmsc388z/assignments/blob/main/assignment3/assignment3.md<br/> 
https://docs.rs/clap/2.33.3/clap/
