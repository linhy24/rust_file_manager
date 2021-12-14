#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        use clap::{App, Arg}; // tell Rust you will use these two structs in clap
        use lib::{run_find, FindConfig}; // tell Rust you will use these two things from our "lib" module
        let matches = App::new("rust_find")
            .version("0.1.0")
            .author("Your Name <you.email@umd.edu>") 
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
            // TODO: specify --dir here
            .arg(
                Arg::from("-d, --dirs=<dirs> 'Set of directories'")
                    .takes_value(true)
                    .required(true)
                    .multiple_values(true),
            )
            // TODO: specify --patterns here
            .arg(
                Arg::from("-s, --size=<size> 'Minimum size in bytes that a matched file needs to have to be reported'")
                    .takes_value(true)
                    .required(false),
            )
//            .get_matches();
            .get_matches_from(vec!["rust-find", "--patterns=.*/.rs", "--output=./tests.out", "--dirs", "./src", "./tests"]);

        let args = FindConfig::from_args(&matches); // will be defined later

        if let Err(err) = run_find(&args) {
            //Error handling here!
            panic!("{}", err)
        }
        //TODO: assert the args
        assert!(true);
    }
    #[test]
    fn test_parse_dirs() {
        use lib::FindConfig;
        let dirs = vec!["."];
        let patterns = vec!["."];
        // all ok
        let cfg = FindConfig {
            dirs,
            patterns,
            output: None,
            size: None,
            exec: None,
            replace: None,
            all: false,
        };
        let res = cfg.parse_dirs();
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 1);

        // one ok
        let cfg2 = FindConfig {
            dirs: vec![".", "/dev/null"],
            ..cfg
        };
        let res2 = cfg2.parse_dirs();
        assert!(res2.is_ok());
        assert_eq!(res2.unwrap().len(), 1);

        let cfg3 = FindConfig {
            dirs: vec!["/dev/null"],
            ..cfg2
        };
        let res3 = cfg3.parse_dirs();
        assert!(res3.is_err());
    }

    #[test]
    fn test_parse_size() {
        use lib::FindConfig;
        let dirs = vec!["/dev/null"];
        let patterns = vec!["."];
        // expect some
        let size = Some("1");
        let cfg = FindConfig {
            dirs,
            patterns,
            output: None,
            size: None,
            exec: None,
            replace: None,
            all: false,
        };
        let res = cfg.parse_size();
        assert!(res.is_some());

        // number should be positive
        let size = Some("-1");
        let cfg2 = FindConfig { size, ..cfg };
        let res2 = cfg2.parse_size();
        assert!(res2.is_none());

        // number should not be float
        let size = Some("1.1");
        let cfg3 = FindConfig { size, ..cfg2 };
        let res3 = cfg3.parse_size();
        assert!(res3.is_none());

        // number should be integer
        let size = Some("a");
        let cfg4 = FindConfig { size, ..cfg3 };
        let res4 = cfg4.parse_size();
        assert!(res4.is_none());
    }

    #[test]
    fn test_parse_patterns() {
        use lib::FindConfig;
        let dirs = vec![".", ".."];
        let patterns = vec!["."];
        // all valid regex
        let cfg = FindConfig {
            dirs,
            patterns,
            output: None,
            size: None,
            exec: None,
            replace: None,
            all: false,
        };
        let res = cfg.parse_patterns();
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 1);

        // some invalid regex
        let patterns = vec![".", ")"];
        let cfg2 = FindConfig { patterns, ..cfg };
        let res2 = cfg2.parse_patterns();
        assert!(res2.is_ok());
        assert_eq!(res2.unwrap().len(), 1);

        // no valid regex
        let patterns = vec![")"];
        let cfg3 = FindConfig { patterns, ..cfg2 };
        let res3 = cfg3.parse_patterns();
        assert!(res3.is_err());
    }
    // TODO: test creating MyFile and get_matched_files
    #[test]

    fn test_myfile() {
        use lib::MyFile;
        use std::path::PathBuf;
        // should work on file
        let path = PathBuf::from("/dev/null");
        let myfile = MyFile::from_path(&path);
        assert!(myfile.is_ok());

        // fail on directory
        let path = PathBuf::from(".");
        let myfile = MyFile::from_path(&path);
        assert!(myfile.is_err())
    }
}
