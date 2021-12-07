/*
This file is to create custom errors and wrap all other existing error crates
through one 'interface'
*/
use std::io;
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    MissingArguments,
    MissingDirectory(String),
    UnknownArgument(String),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Error::MissingArguments =>
                write!(f, "Not enough arguments.\nThe tool requires '-dir0' '-dir1' and '-output'."),
            Error::MissingDirectory(directory) =>
                write!(f, "The directory '{}' does not exists.", directory),
            Error::UnknownArgument(argument) =>
                write!(f, "Unknown argument '{}'.", argument),
            Error::Io(io_error) =>
                write!(f, "{}", io_error),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &*self {
            Error::MissingArguments => None,
            Error::MissingDirectory(_) => None,
            Error::UnknownArgument(_) => None,
            Error::Io(_) => None,
        }
    }
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::Io(err)
	}
}
