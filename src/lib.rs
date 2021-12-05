use std::fs;
use std::path::Path;
use std::error::Error;

use walkdir::WalkDir;


// Structure with all the commands managed by the tool
#[derive(Debug)]
pub struct Commands {
    pub dir0: String,           // path of the dir we want to search
    pub dir1: String,           // path of the dir we want to search
    pub output: String,         // path of the output dir were new mod is located
    pub common: String
}

// Default structure for Commands so we can initialize an "Empty" Commands struct
impl Default for Commands{
    fn default () -> Commands {
        Commands{
            dir0:   String::from("./"),
            dir1:   String::from("./"),
            output: String::from("./"),
            common: String::from("/romfs")
        }
    }
}

// Implement an iterator to Commands

// Get the vector with the command line arguments
impl Commands {
    // Commands constructor
    pub fn new(args:&[String]) -> Result<Commands, String> {

        let mut commands = Commands::default();

        // Less then 3 arguments shouldn't be permitted, you need -dir0, -dir1 and -output for the
        // program to work
        if args.len() < 3 {
            return Err(String::from("Not enough arguments")); // should show help page
        }

        // Iterate from index 0 to the end of the vector
        for arg in &args[1..]{
            // split the argument, map them to string and collect them to make a vector
            // of type [cmd, value]
            let splitted_arg: Vec<String> = arg.split('=').map(|s| s.to_string()).collect();

            // match the inputs to the Commands struct
            match &*splitted_arg[0] {
                "-dir0"     => commands.dir0 = splitted_arg[1].clone(),
                "-dir1"     => commands.dir1 = splitted_arg[1].clone(),
                "-output"   => commands.output = splitted_arg[1].clone(),
                "-common"   => commands.common = splitted_arg[1].clone(),
                _           => return Err(format!("Unknown argument '{}'",&splitted_arg[0])),
            }
        }

        // Return the Commands struct
        Ok(commands)
    }
}

// Validate that the given directories exist
pub fn valid_dir(commands: &Commands) -> Result<(), String>{
    // A way to make this better is to implement an iterator for the Commands struct and iterate
    // through every entry to validate their existence, this eliminates duplicated code
    if !Path::new(&commands.dir0).exists(){
        return Err(format!("Directory '{}' does not exists.", &commands.dir0))
    }
    if !Path::new(&commands.dir1).exists(){
        return Err(format!("Directory '{}' does not exists.", &commands.dir1))
    }
    if !Path::new(&commands.output).exists(){
        return Err(format!("Directory '{}' does not exists.", &commands.output))
    }
    Ok(())
}

// Search dir0 and dir1 for files they have in common and files that they don't.
// If they don't exist in each folder tree merge them and put it on output.
// It's like making an outer join of dir0 and dir1 to output.
// If they do exist in each folder tree then <I'm fucked>
pub fn merge(commands: &Commands) -> Result<(), Box<dyn Error>>{
    // Creates an iterator with every leaf of the directory tree
    let dir0 = WalkDir::new(&commands.dir0).into_iter().filter_map(|e| e.ok());
    let dir1 = WalkDir::new(&commands.dir1).into_iter().filter_map(|e| e.ok());

    // list of files that don't exist in both directories
    let mut outer_join_files:Vec<(&String, String)> = vec![];

    // list of files that do exist in both directories
    let mut inner_join_files:Vec<(&String, &String, String)> = vec![];

    // To do: remove duplicated code in both for loops
    for entry in dir0{
        if entry.path().is_file(){
            // Separate the input folder dir from the file dir
            let file = entry.path().to_str().unwrap().replace(&commands.dir0,"");
            let file_dir = (&commands.dir0, file.clone());
            if !outer_join_files.contains(&file_dir){
                outer_join_files.push((&commands.dir0, file));
            }
        }
    }

    for entry in dir1{
        if entry.path().is_file(){
            let file = entry.path().to_str().unwrap().replace(&commands.dir1,"");
            let file_dir = (&commands.dir0, file.clone());
            if !outer_join_files.contains(&file_dir){
                outer_join_files.push((&commands.dir1, file));
            }
            else{
                inner_join_files.push((&commands.dir0, &commands.dir1, file));
                let index = outer_join_files.iter().position(|x| *x == file_dir).unwrap();
                let file_rm = outer_join_files.get(index);
                println!("{:?}", &file_rm);
                outer_join_files.remove(index);
            }
        }
    }

    // Add a new folder to the output dir
    let output = Path::new(&commands.output).join("merged");

    // For every file in the outer join create the parent dir in output if it doesn't exist then
    // copy the file to the new directory.
    for file in outer_join_files.iter(){
        // Origin of the file
        let from = format!("{}{}",file.0,file.1);
        // Output dir
        let to = format!("{}{}",output.to_str().unwrap(),file.1);
        // Parent dir of the file in the output dir
        let to_parent = Path::new(&to).parent().unwrap();

        // If the origin folder is not the same as the output folder.
        // This to avoid corrupting the data of the output folder when trying to copy
        // over the same file
        if &file.0 != &output.to_str().unwrap() {
            // Create the output dir if it doesn't exist
            if !to_parent.is_dir(){
                // To do: needs better error handling (catch the error and throw a more verbose error)
                fs::create_dir_all(&to_parent.to_str().unwrap())?;
            }

            // Copy the file to the output dir
            // To do: needs better error handling (catch the error and throw a more verbose error)
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
            println!("'{}{}'\n'{}{}'", &file.0, &file.2, &file.1, &file.2);
        }
    }

    Ok(())
}
