use std::env;
use std::ffi::OsString;
use std::io::{self, Write};
use std::sync::Arc;
use std::process;


use clap;

use crate::Result;
use crate::app;

/// The primary configuration object used throughout jq-rs. It provides a       
/// high-level convenient interface to the provided command line arguments.
///
/// An `Args` object is cheap to clone and can be used from multiple threads
/// simultaneously.
#[derive(Clone, Debug)]     
pub struct Args(Arc<ArgsImp>);


#[derive(Clone, Debug)]     
pub struct ArgsImp {
    matches: ArgMatches,
}

impl Args {
    pub fn parse() -> Result<Args> {
        let matches = ArgMatches::new(clap_matches(env::args_os())?);

        matches.to_args()
    }

    pub fn matches(&self) -> &ArgMatches {
        &self.0.matches
    }

    pub fn verbose(&self) -> bool {
        self.0.matches.0.is_present("verbose")
    }

    pub fn debug(&self) -> bool {
        self.0.matches.0.is_present("debug")
    }

    pub fn quiet(&self) -> bool {
        self.0.matches.0.is_present("quiet")
    }

    pub fn input(&self) -> &str {
        &self.0.matches.0.value_of("INPUT FILE").unwrap_or("<stdin>")
    } 
}

#[derive(Clone, Debug)]
pub struct ArgMatches(clap::ArgMatches<'static>);

impl ArgMatches {
    fn new(clap_matches: clap::ArgMatches<'static>) -> Self {
        Self(clap_matches)
    }

    fn to_args(self) -> Result<Args> {
        Ok(Args(Arc::new(ArgsImp {
            matches: self,
        })))
    }
}

/// Returns a clap matches object if the given arguments parse successfully.
///                                                         
/// Otherwise, if an error occurred, then it is returned unless the error
/// corresponds to a `--help` or `--version` request. In which  case, the     
/// corresponding output is printed and the current process is exited
/// successfully.
fn clap_matches<I, T>(args: I) -> Result<clap::ArgMatches<'static>>
where
    I: IntoIterator<Item = T>,         
    T: Into<OsString> + Clone, 
{
    let err = match app::app().get_matches_from_safe(args) {
        Ok(matches) => return Ok(matches),            
        Err(err) => err, 
    };
    if err.use_stderr() {
        return Err(err.into());
    }
    // Explicitly ignore any error returned by write!. The most likely error    
    // at this point is a broken pipe error, in which case, we want to ignore
    // it and exit quietly.
    //
    // (This is the point of this helper function. clap's functionality for       
    // doing this will panic on a broken pipe error.)               
    let _ = write!(io::stdout(), "{}", err);
    process::exit(0);            
}
