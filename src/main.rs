/*
I don't know shit about licenses.

The idea is to hopefully create a crossplatform mod merger for Pokemon BDSP mods, because I'm a
masochistic and want to learn rust to do crossplatform tools/projects, hopefully the tool
detects the files that don't exist in each mod and merges them. Just to kill myself, when the same
asset bundle file exists in both mods it should decompress the bundle, find the modified files, and
then merge the changes. If both mods edit the same file of the bundle then fuck me. I don't think
this will ever work.😂

1.  Add the "merge" dir to the output dir.

2.  Get the traversal iterator for both folders.

3.  Make a list of the files in the outer join of both folders.
    It's a tuple with the form of (folder,file).

4.  Make a list of all the files in the in the inner join of of both folder.
    It's a tuple with the form of (dir0, dir1, file)

5.  Copy al the files from step 3 to the output dir

6.  Compare every file readable by diffy (https://docs.rs/diffy/latest/diffy/) and use it to create
    a type of version control for the files.

7.  Decompress the assetbundle for the files in step 4, compare every file, when a file is edited
    merge them if possible, recompress the bundle and move it the output dir.
    (I don't know if this is possible without an enormous amount of effort)

7.  Decompress any compressed file, compare every file in the decompressed bundle,
    when a file is edited merge them if possible, recompress the bundle and move it the output dir.
    (I don't know if this is possible without an enormous amount of effort)
*/

use std::env;
use std::process;

use dir_merger::Commands;

fn main() {
    // read any command line arguments passed to it and then collect the values into a vector
    let args: Vec<String> = env::args().collect();

    // Parse the commands from the passed command line arguments
    let commands = Commands::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // validate that al given directories exist
    dir_merger::valid_dir(&commands).unwrap_or_else(|err| {
        eprintln!("Validation error: {}", err);
        process::exit(1);
    });

    // Merge the folder trees
    match dir_merger::merge(&commands){
        Ok(()) => println!("\nMerge completed without errors!"),
        Err(err) => {
            eprintln!("Merge error: {}", err);
            process::exit(1);
        }
    }
}
