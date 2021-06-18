# `rust-fm`
`rust-fm` is a cli-file manager written in Rust. Just type, and get the files.(fzf)

## What it can do
* Navigate in the folder structure of the system
* Typing shows only the elements you are interested in using fuzzy finding
* Preview of folders and (text)files
* Opening files in $EDITOR or using xdg-open
* Copy-Paste(Not Tested)

## What it cannot do yet
* Basic file and folder actions like renaming, deleting, ...
* Create a file or folder

## How to install it
* Clone the repository
* Build the binary using cargo build (rust and cargo have to be installed, see [installation guide](https://www.rust-lang.org/tools/install))
* Binary should be target/debug/rust-fm

## Usage
Use Arrow Keys to move around(see key_bindings.md). 
Just type, and it will show only the files, and directories which has the words/lettersyou have typed.
Press <esc> to reload(show all files).
Ctrl-C Quits the application. 

## Additional Features

### Copy-Paste
To copy and paste filesor folders you first have to highlight the files you want. To highlight files press the space bar when selecting the item. You can highlight many items at once. Now you have to yank your files or folders using 'Y'. Now move to the folder you want those items to be and press 'P' to paste.

### Instant Viewing of Files
When you are moving around in your directories, and suddenly see a file you would like to change!
Simply click on the right arrow key(or enter), and it will open in your $EDITOR. It also uses xdg-open to open files.

### Shows the path(pwd)
At the top of the buffer, it shows the path, therefore being able to see where you are going.

## Changes from Schmiddiii/rust-fm
* Changed the colorscheme. In Schmiddiii/rust-fm, files(selected) are shown as white on white. Thus, we can't see it. Now Changed to Black on white.
* Changed the keybindings, as it would be complicated to type Uppercase/Capital letters to move around. 
* Quitting by Ctrl-c or Shift-Q or Ctrl-q.

## Requirements
* xdg-open
```sh
sudo apt install xdg-utils
```

### Extra Requirements(Just need to change the following)
* Set EDITOR in your .bashrc(or whichever prompt you have) at the end by 
```sh
export EDITOR=<Editor of your choice> #kak, vim, vi, nano, etc
```
