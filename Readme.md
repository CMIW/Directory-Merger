# Directory Merger

![GitHub repo size](https://img.shields.io/github/repo-size/CMIW/Directory-Merger)
![GitHub contributors](https://img.shields.io/github/contributors/CMIW/Directory-Merger)
![GitHub stars](https://img.shields.io/github/stars/CMIW/Directory-Merger?style=social)
![GitHub forks](https://img.shields.io/github/forks/CMIW/Directory-Merger?style=social)

This is a simple command line tool, build with Rust, to merge the contents of distinct folders. It was made for learning and educational purpose. It's trash but it's trash I'm proud of.

It won't merge files with the same name, it only merges the differences between directories. It takes as arguments the directories of the folders you want to merge and the directory of where the merged folder should be.

## Using Directory Merger

No installation is required. Download the binaries for your system from the latest release https://github.com/CMIW/Directory-Merger/releases. Extract the binaries from the zip file. Open a terminal where you have the binaries and run the following command:
```
dir_merger -dir0=<folder path> -dir1=<folder path> -output=<path>
```

This will create a folder named "merged" in the output directory with the files from the input folders.

Linux example:
```
./dir_merger -dir0=/home/user/Downloads/wild_encounters -dir1=/home/user/Downloads/No_Trade_Evolutions -output=/home/user/Downloads
```

Windows example:
```
dir_merger.exe -dir0=C:\Users\user\Downloads\wild_encounters -dir1=C:\Users\user\Downloads\No_Trade_Evolutions -output=C:\Users\user\Downloads
```
## Building Directory Merger

Follow the Rust installation guide from https://doc.rust-lang.org/book/ch01-01-installation.html.

Clone the project repo:

HTTPS:
```
https://github.com/CMIW/Directory-Merger.git
```

SSH:
```
git@github.com:CMIW/Directory-Merger.git
```

GitHub CLI:
```
gh repo clone CMIW/Directory-Merger
```
Open a terminal, move to the project folder and run:
```
cargo build
```
