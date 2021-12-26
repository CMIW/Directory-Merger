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
    UnknownArgument(String),
    MergingConflict(String),
    MissingDirectory(String),
    Io(io::Error),
    PyError(cpython::PyErr),
    JsonError(serde_json::Error),
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
            Error::MergingConflict(file_name) =>
                write!(f, "Conflict merging {}, both files modified the same region of the \
                    original class, changes will be ignored.\n",
                    file_name
                ),
            Error::PyError(py_error) =>
                write!(f, "{:?}", py_error),
            Error::JsonError(error) =>
                write!(f, "{}", error),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &*self {
            Error::MissingArguments     => None,
            Error::MissingDirectory(_)  => None,
            Error::UnknownArgument(_)   => None,
            Error::Io(_)                => None,
            Error::MergingConflict(_)   => None,
            Error::PyError(_)           => None,
            Error::JsonError(_)         => None,
        }
    }
}

// Implementing diferent external kind of Error that will be used

// Implementing IOError for the custom error enumrator
impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
		Error::Io(err)
	}
}

// Implementing PyError for the custom error enumrator
impl From<cpython::PyErr> for Error {
	fn from(err: cpython::PyErr) -> Error {
		Error::PyError(err)
	}
}

// Implementing serde_json errors for the custom error enumrator
impl From<serde_json::Error> for Error {
	fn from(err: serde_json::Error) -> Error {
		Error::JsonError(err)
	}
}
