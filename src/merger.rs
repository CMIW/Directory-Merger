use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use walkdir::DirEntry;

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
    Ok(())
}

//
pub fn merge_aux(outer_files: &mut Vec<File>, inner_files: &mut Vec<File>, entry: &DirEntry, dir: &str){
    if entry.path().is_file(){
        // Separate the input folder dir from the file dir
        let file_dir = entry.path().to_str().unwrap().replace(dir,"");
        // Build file struct
        let mut file = File::new(dir, "", &file_dir);
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
