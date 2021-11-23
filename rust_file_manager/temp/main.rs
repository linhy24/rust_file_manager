use clap::{App, Arg}; // tell Rust you will use these two structs in clap
use lib::{run_find, run_add, FindConfig, AddConfig}; // tell Rust you will use these two things from our "lib" module

fn main() {
    // Define command-line interface below

    // command specification for add
    let add_command = App::new("rust_add")
        .version("0.1.0")
        .author("linhy@terpmail.umd.edu")
        .about("Add files with specified file name")
        .arg(
            Arg::from("-a, --add=<filename> 'Name of file to add'")
                .takes_value(true)
                .required(true), //.multiple_values(true)
        )
        .arg(
            Arg::from("-d, --dirs=<dirs> 'Specify the directory that you want to add file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    // .get_matches_from(vec!["rust-add", "--add=", "--dirs=./"]);

    // command specification for find
    let find_command = App::new("rust_find")
        .version("0.1.0")
        .author("linhy@terpmail.umd.edu") 
        .about("Find files that match a regex pattern")
        .arg(
            Arg::from("-p , --patterns=<patterns> 'List of file patterns to find.'")
                .takes_value(true)
                .required(true)
                .multiple_values(true), // this argument can takes multiple values
        )
        .arg(
            Arg::from("-o, --output=<output> 'Write results to output file instead of stdout.'")
                .takes_value(true) // argument if true or flag if false.
                .required(false), // this is an optional argument
        )
        .arg(
            Arg::from("-d, --dirs=<dirs> 'Specify a set of directories that your program will find the matched filenames from.'")
                .takes_value(true)
                .required(true)
                .multiple_values(true)
        )
        .arg(
            Arg::from("-s, --size=<size> 'Specify the minimum size, in bytes, a matched files need to have to be reported.'")
                .required(false)
        )
        // .get_matches();
        .get_matches_from(vec!["rust-find", "--patterns=.*\\.txt", "--dirs=./"]);

    let find_args = FindConfig::from_args(&find_command); // will be defined later
    let add_args = AddConfig::from_args(&add_command);

    if let Err(err) = run_find(&find_args) {
        //Error handling here!
        panic!("{}", err)
    }

    if let Err(err) = run_add(&add_args) {
        //Error handling here!
        panic!("{}", err)
    }
}

