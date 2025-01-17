use std::ffi::OsString;
use std::io;
use std::io::Write;

use ucd_parse::{UcdFile, UnicodeData};

use crate::args::ArgMatches;
use crate::common::err;
use crate::{
    age, app, bidi_class, bidi_mirroring_glyph, brk,
    canonical_combining_class, case_folding, case_mapping, error,
    general_category, jamo_short_name, joining_type, names, property_bool,
    script,
};

/// Run ucd-generate, with the provided "command line" arguments.
pub fn run<I, T>(matches: I) -> error::Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = app::app().get_matches_from(matches);
    match matches.subcommand() {
        ("bidi-class", Some(m)) => bidi_class::command(ArgMatches::new(m)),
        ("bidi-mirroring-glyph", Some(m)) => {
            bidi_mirroring_glyph::command(ArgMatches::new(m))
        }
        ("canonical-combining-class", Some(m)) => {
            canonical_combining_class::command(ArgMatches::new(m))
        }
        ("general-category", Some(m)) => {
            general_category::command(ArgMatches::new(m))
        }
        ("script", Some(m)) => script::command_script(ArgMatches::new(m)),
        ("script-extension", Some(m)) => {
            script::command_script_extension(ArgMatches::new(m))
        }
        ("property-bool", Some(m)) => {
            property_bool::command(ArgMatches::new(m))
        }
        ("age", Some(m)) => age::command(ArgMatches::new(m)),
        ("perl-word", Some(m)) => {
            property_bool::command_perl_word(ArgMatches::new(m))
        }
        ("jamo-short-name", Some(m)) => {
            jamo_short_name::command(ArgMatches::new(m))
        }
        ("joining-type", Some(m)) => joining_type::command(ArgMatches::new(m)),
        ("names", Some(m)) => names::command(ArgMatches::new(m)),
        ("property-names", Some(m)) => cmd_property_names(ArgMatches::new(m)),
        ("property-values", Some(m)) => {
            cmd_property_values(ArgMatches::new(m))
        }
        ("case-folding-simple", Some(m)) => {
            case_folding::command(ArgMatches::new(m))
        }
        ("case-mapping", Some(m)) => case_mapping::command(ArgMatches::new(m)),
        ("grapheme-cluster-break", Some(m)) => {
            brk::grapheme_cluster(ArgMatches::new(m))
        }
        ("word-break", Some(m)) => brk::word(ArgMatches::new(m)),
        ("sentence-break", Some(m)) => brk::sentence(ArgMatches::new(m)),
        ("test-unicode-data", Some(m)) => {
            cmd_test_unicode_data(ArgMatches::new(m))
        }
        ("", _) => {
            app::app().print_help()?;
            println!();
            Ok(())
        }
        (unknown, _) => err!("unrecognized command: {}", unknown),
    }
}

fn cmd_property_names(args: ArgMatches<'_>) -> error::Result<()> {
    use crate::util::PropertyNames;
    use std::collections::BTreeMap;

    let dir = args.ucd_dir()?;
    let names = PropertyNames::from_ucd_dir(&dir)?;
    let filter = args.filter(|name| names.canonical(name))?;

    let mut actual_names = BTreeMap::new();
    for (k, v) in &names.0 {
        if filter.contains(v) {
            actual_names.insert(k.to_string(), v.to_string());
        }
    }
    let mut wtr = args.writer("property_names")?;
    wtr.string_to_string(args.name(), &actual_names)?;
    Ok(())
}

fn cmd_property_values(args: ArgMatches<'_>) -> error::Result<()> {
    use crate::util::{PropertyNames, PropertyValues};
    use std::collections::BTreeMap;

    let dir = args.ucd_dir()?;
    let values = PropertyValues::from_ucd_dir(&dir)?;
    let names = PropertyNames::from_ucd_dir(&dir)?;
    let filter = args.filter(|name| names.canonical(name))?;

    let mut actual_values = BTreeMap::new();
    for (k, v) in &values.value {
        if filter.contains(k) {
            actual_values.insert(k.to_string(), v.clone());
        }
    }
    let mut wtr = args.writer("property_values")?;
    wtr.string_to_string_to_string(args.name(), &actual_values)?;
    Ok(())
}

fn cmd_test_unicode_data(args: ArgMatches<'_>) -> error::Result<()> {
    let dir = args.ucd_dir()?;
    let mut stdout = io::stdout();
    for result in UnicodeData::from_dir(dir)? {
        let x: UnicodeData = result?;
        writeln!(stdout, "{}", x)?;
    }
    Ok(())
}
