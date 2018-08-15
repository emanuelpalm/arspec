mod error;

pub use self::error::Error;

use ahfs::graph::{Graph, Query, Triple};
use ahfs::log;
use ahfs::parser;
use ahfs::project::Project;
use ahfs::source::Source;
use std::io;

/// Creates new project at first path in `args` and exits.
pub fn new(args: &[&str], ignore_if_exists: bool) -> Result<(), Box<::ahfs::ErrorCode>> {
    if args.len() != 1 {
        return Err(Error::NewArgCountNot1.into());
    }
    match Project::create(args[0].clone()) {
        Ok(_) => Ok(()),
        Err(err) => {
            if ignore_if_exists {
                let ignore = err.as_io_error().map_or(false, |err| {
                    err.kind() == io::ErrorKind::AlreadyExists
                });
                if ignore {
                    return Ok(());
                }
            }
            Err(err)
        }
    }
}

/// Generates graph files from all project source files and exits.
pub fn graph(args: &[&str]) -> Result<(), Box<::ahfs::ErrorCode>> {
    if args.len() != 0 {
        return Err(Error::GraphArgCountNot0.into());
    }
    let project = Project::locate(".")?;
    let files = project.files()?;
    let source = Source::read_files(files.iter())?;
    let triples = parser::parse(&source)?;

    // TODO: Actually generate graph files.
    let types: Vec<&Triple> = triples.query()
        .predicate("ahfs.type")
        .collect();
    for triple in types {
        println!("{}", triple.predicate());
    }

    Ok(())
}

/// Prints list of all project source files and exits.
pub fn list(args: &[&str]) -> Result<(), Box<::ahfs::ErrorCode>> {
    if args.len() != 0 {
        return Err(Error::ListArgCountNot0.into());
    }
    let project = Project::locate(".")?;
    let files = project.files()?;
    for file in files.iter() {
        log::completion(&file.canonicalize()?.to_string_lossy());
    }
    log::completion(&format!("Files found: {}", files.len()));
    Ok(())
}

/// Prints various project status information and exits.
pub fn status(args: &[&str]) -> Result<(), Box<::ahfs::ErrorCode>> {
    if args.len() != 0 {
        return Err(Error::StatusArgCountNot0.into());
    }
    let project = Project::locate(".")?;
    log::completion(&format!("Project:      {}", project.root().canonicalize()?
        .file_name().unwrap().to_string_lossy()));
    log::completion(&format!("AHFS Version: {}", project.settings()
        .ahfs_version()));
    Ok(())
}