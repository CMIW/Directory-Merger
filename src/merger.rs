use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use walkdir::DirEntry;
use serde_json::Value;
use diffy::merge as diffy_merge;

use crate::unity::{python_decompress_unity_fs};
use crate::file::File;
use crate::error::Error;
use crate::commands::Commands;


// Validate that the given directories exist
pub fn valid_dir(commands: &Commands) -> Result<(), Error>{
    if !Path::new(&commands.dir0).exists(){
        return Err(Error::MissingDirectory(commands.dir0.clone()))
    }
    if !Path::new(&commands.dir1).exists(){
        return Err(Error::MissingDirectory(commands.dir1.clone()))
    }
    if !Path::new(&commands.output).exists(){
        return Err(Error::MissingDirectory(commands.output.clone()))
    }
    if !Path::new(&commands.original).exists(){
        return Err(Error::MissingDirectory(commands.original.clone()))
    }
    Ok(())
}

// Search dir0 and dir1 for files they have in common and files that they don't.
// If they don't exist in each folder tree merge them and put it on output.
// It's like making an outer join of dir0 and dir1 to output.
// If they do exist in each folder tree then <I'm fucked>
pub fn merge(commands: &Commands) -> Result<(), Error>{
    // Creates an iterator with every leaf of the directory tree
    let dir0 = WalkDir::new(&commands.dir0).into_iter().filter_map(|e| e.ok());
    let dir1 = WalkDir::new(&commands.dir1).into_iter().filter_map(|e| e.ok());

    // list of files that don't exist in both directories
    let mut outer_join_files:Vec<File> = vec![];

    // list of files that do exist in both directories
    let mut inner_join_files:Vec<File> = vec![];

    for entry in dir0{
        merge_aux(&mut outer_join_files, &mut inner_join_files, &entry, &commands.dir0);
    }

    for entry in dir1{
        merge_aux(&mut outer_join_files, &mut inner_join_files, &entry, &commands.dir1);
    }

    // Add a new folder to the output dir
    let output = Path::new(&commands.output).join("merged");

    // For every file in the outer join create the parent dir in output if it doesn't exist then
    // copy the file to the new directory.
    for file in outer_join_files.iter(){
        // Origin of the file
        let from = format!("{}{}",file.dir0,file.file);
        // Output dir
        let to = format!("{}{}",output.to_str().unwrap(),file.file);
        // Parent dir of the file in the output dir
        let to_parent = Path::new(&to).parent().unwrap();

        // If the origin folder is not the same as the output folder.
        // This to avoid corrupting the data of the output folder when trying to copy
        // over the same file
        if &file.dir0 != &output.to_str().unwrap() {
            // Create the output dir if it doesn't exist
            if !to_parent.is_dir(){
                fs::create_dir_all(&to_parent.to_str().unwrap())?;
            }

            // Copy the file to the output dir
            fs::copy(from, to)?;
        }
    }

    // When the vector is not empty show the files that can't be merged
    // because they are in both folders
    if inner_join_files.len() > 0 {
        println!(
            "\nThe following files can't be merged, the tool doesn't know which files to keep:"
        );

        for file in inner_join_files.iter() {
            println!("'{}{}'\n'{}{}'", &file.dir0, &file.file, &file.dir1, &file.file);
        }
    }

    Ok(())
}

//
pub fn merge_aux(outer_files: &mut Vec<File>, inner_files: &mut Vec<File>, entry: &DirEntry, dir: &String){
    if entry.path().is_file(){
        // Separate the input folder dir from the file dir
        let file_dir = entry.path().to_str().unwrap().replace(dir,"");
        // Build file struct
        let mut file = File::new(dir.to_owned(), "".to_owned(), file_dir);
        if outer_files.contains(&file){
            // get the index of the file
            let index = outer_files.iter().position(|x| *x == file).unwrap();

            // get ownership of the file by removing it from the vector
            file = outer_files.remove(index);

            // modify the file to add the second dir
            file.dir1 = dir.to_owned();

            // push to the inner join vector
            inner_files.push(file);
        }
        else{
            outer_files.push(file);
        }
    }
}

// Try comparing the same bundle file from different mods
fn patch_unityfs(original:&str, base:&str, _mod:&str) -> Result<Vec<Value>, Error> {
    println!("Decompressing Files...");

    // Data of the files in the unity bundle in the form of String
    let orig_decompressed = python_decompress_unity_fs(&original).unwrap();
    let base_decompressed = python_decompress_unity_fs(&base).unwrap();
    let mod_decompressed = python_decompress_unity_fs(&_mod).unwrap();

    let mut changes: Vec<Value> = [].to_vec();

    // Compare every class in both unity bundle files
    for i in 0..base_decompressed.len() {
        for j in 0..mod_decompressed.len() {
            // Only do comparisons on the same class
            if &base_decompressed[i]["m_Name"] == &mod_decompressed[j]["m_Name"] {
                // Show the classes that are not the same
                if &base_decompressed[i] != &mod_decompressed[j]{
                    // Convert the class to a string to make the comparisons
                    let base_str = base_decompressed[i].to_string();
                    let mod_str = mod_decompressed[i].to_string();

                    // Find the class we need to compare in the original romfs
                    let ori_str: String = match find_unity_class(
                        &orig_decompressed,
                        "m_Name",
                        &base_decompressed[i]["m_Name"].to_string()
                    ) {
                        None => {break;},
                        Some(result) => {result.to_string()},
                    };

                    // If possible merge the differences between the classes
                    // Try to Merge al differences
                    let patch = match diffy_merge(&ori_str, &base_str, &mod_str) {
                        Ok(result) => result,
                        Err(_) => {
                            println!("{}",
                                Error::MergingConflict(base_decompressed[i]["m_Name"].to_string())
                            );
                            break;
                        },
                    };

                    // create a json from the merged result
                    let merged_class: Value = serde_json::from_str(&patch)?;

                    println!("Applying changes to {}.\n", &merged_class["m_Name"]);

                    // Add the classes with changes to a vector
                    changes.push(merged_class);
                }
            }
        }
    }

    Ok(changes)
}

// Find json by atribute in a given vector of json objects
fn find_unity_class(vector:&Vec<Value>, atribute:&str, value:&str) -> Option<Value> {
    for i in 0..vector.len() {
        if vector[i][atribute].to_string() == value {
            return Some(vector[i].clone())
        }
    }
    None
}

// Try to extract the extension from the file.
// When there is no extension on the file get the file type from the binary header.
fn get_file_type(file:&str) -> Result<String, Error> {
    // Extract the extension from the file.
    match Path::new(file).extension() {
        None => (),
        Some(file_extension) =>
            return Ok(file_extension.to_ascii_uppercase().into_string().unwrap()),
    }

    // Get the file type from the binary header.
    // Read bytes from the binary
    let bytes = std::fs::read(file)?;
    // Create emtpy string to store the type
    let mut file_type = String::from("");

    // Iterate every byte, convert the u8 byte to char, extract every value that's alphabetic until
    // you find garbage
    for (index, byte) in bytes.iter().enumerate() {
        if (**&byte as char).is_alphabetic() {
            file_type.push(*&bytes[index] as char);
        }
        else if index > 0{
            break;
        }
    }

    Ok(file_type)
}
