#[derive(Debug)]
pub struct File {
    pub dir0: String,           // Directory to the input folder
    pub dir1: String,           // Directory to the input folder
    pub file: String,           // Directory to the file without the directory to the input folder
}

impl File {
    // File constructor
    pub fn new(dir0:String, dir1:String, file:String) -> File {

        File {dir0: dir0, dir1: dir1, file: file}
    }
}

// Implement the "==" so the struct can be compared 
impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.file == other.file
    }
}

impl Eq for File {}
