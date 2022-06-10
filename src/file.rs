#[derive(Debug)]
pub struct File {
    pub dir0: String,           // Directory to the input folder
    pub dir1: String,           // Directory to the input folder
    pub file: String,           // Directory to the file without the directory to the input folder
}

impl File {
    // File constructor
    pub fn new(dir0:&str, dir1:&str, file:&str) -> File {

        File {dir0: dir0.to_string(), dir1: dir1.to_string(), file: file.to_string()}
    }
}

// Implement the "==" so the struct can be compared
impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.file == other.file
    }
}

impl Eq for File {}
