// Structure with all the commands managed by the tool
#[derive(Debug)]
pub struct Commands {
    pub dir0: String,           // path of the dir we want to search
    pub dir1: String,           // path of the dir we want to search
    pub output: String          // path of the output dir were new mod is located
}

// Default structure for Commands so we can initialize an "Empty" Commands struct
impl Default for Commands{
    fn default () -> Commands {
        Commands{
            dir0:   String::from("./"),
            dir1:   String::from("./"),
            output: String::from("./")
        }
    }
}

// Get the vector with the command line arguments
impl Commands {
    // Commands constructor
    pub fn new(args:&[String]) -> Result<Commands, String> {

        let mut commands = Commands::default();

        // Less then 3 arguments shouldn't be permitted, you need -dir0, -dir1 and -output for the
        // program to work
        if args.len() < 3 {
            return Err(String::from(
                "Not enough arguments.\nThe tool requires '-dir0' '-dir1' and '-output'."
            )); // should show help page
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
                _           => return Err(format!("Unknown argument '{}'",&splitted_arg[0])),
            }
        }

        // Return the Commands struct
        Ok(commands)
    }
}

// Implement an iterator to Commands
