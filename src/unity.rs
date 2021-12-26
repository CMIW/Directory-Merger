use crate::error::Error;

use serde_json::Value;
use cpython::{ObjectProtocol, NoArgs, Python};

// Uses python binding to call python modules and objects in Rust
// Opens the unity file and extract its data into vector of json objects
pub fn python_decompress_unity_fs(unity_file:&str) -> Result<Vec<Value>, Error> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    // Import UnityPy
    let unity_py = py.import("UnityPy")?;

    // Import json
    let json = py.import("json")?;

    // Load file via UnityPy.load
    let env = unity_py.call(py, "load", (unity_file,), None)?;

    // Collect the objects into a vector
    let mut decompressed_file_string: Vec<Value> = [].to_vec();

    // Get the list of objects from the unity file
    let objects = env.getattr(py, "objects")?;

    // Convert the list into an iterator
    let objects_iter = objects.iter(py)?;

    // Iterate over internal objects
    for object in objects_iter {
        // Extract the file data to a tree (dictionary)
        let tree = object.unwrap().call_method(py, "read_typetree", NoArgs, None)?;

        // Convert the dictionary to a json
        let tree_json = json.call(py, "dumps", (&tree,), None)?;

        let value: Value = serde_json::from_str(&tree_json.to_string())?;

        // Make the object to String and push it to the result vector
        decompressed_file_string.push(value.clone());

    }

    Ok(decompressed_file_string)
}
