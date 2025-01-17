use std::ffi::OsStr;
use std::ops;

use clap;

use crate::err;
use crate::error::Result;
use crate::util::Filter;
use crate::writer::{Writer, WriterBuilder};

/// Wraps clap matches and provides convenient accessors to various parameters.
pub struct ArgMatches<'a>(&'a clap::ArgMatches<'a>);

impl<'a> ops::Deref for ArgMatches<'a> {
    type Target = clap::ArgMatches<'a>;
    fn deref(&self) -> &clap::ArgMatches<'a> {
        &self.0
    }
}

impl<'a> ArgMatches<'a> {
    pub fn new(matches: &'a clap::ArgMatches<'a>) -> ArgMatches<'a> {
        ArgMatches(matches)
    }

    pub fn ucd_dir(&self) -> Result<&OsStr> {
        match self.value_of_os("ucd-dir") {
            Some(x) => Ok(x),
            None => err!("missing UCD directory"),
        }
    }

    pub fn writer(&self, name: &str) -> Result<Writer> {
        let mut builder = WriterBuilder::new(name);
        builder
            .columns(79)
            .char_literals(self.is_present("chars"))
            .trie_set(self.is_present("trie-set"));
        // Some of the functionality of this crate works with a partial ucd
        // directory.
        match ucd_parse::ucd_directory_version(self.ucd_dir()?) {
            Ok((major, minor, patch)) => {
                builder.ucd_version(major, minor, patch)
            }
            Err(e) => return err!("Failed to determine UCD version: {}", e),
        };
        match self.value_of_os("fst-dir") {
            None => Ok(builder.from_stdout()),
            Some(x) => builder.from_fst_dir(x),
        }
    }

    pub fn name(&self) -> &str {
        self.value_of("name").expect("the name of the table")
    }

    /// Create a new include/exclude filter command line arguments.
    ///
    /// The given canonicalization function is applied to each element in
    /// each of the include/exclude lists provided by the end user.
    pub fn filter<F: FnMut(&str) -> Result<String>>(
        &self,
        mut canonicalize: F,
    ) -> Result<Filter> {
        Filter::new(
            self.value_of_lossy("include").map(|s| s.to_string()),
            self.value_of_lossy("exclude").map(|s| s.to_string()),
            |name| canonicalize(name),
        )
    }
}
