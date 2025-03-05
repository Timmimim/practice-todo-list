/**
 * This file is mostly a 1:1 transcript/copy of the code published by [sioodmy](https://github.com/sioodmy/todo) under a GPL-3.0 license.
 * THANK YOU!
 * 
 * Slightly edited and commented by [timmimim](https://github.com/timmimim).
 * This file has been created for training purposes.
 */

use colored::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::Read;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use std::{env,process};

// outline a struct, cf "object"(-ish)
puc struct Entry {
    pub todo_entry: String,
    pub done: bool,
}

// implement methods for the Entry struct
impl Entry {
    // constructor
    pub fn new(todo_entry: String, done: bool) -> Self {
        Self {
            todo_entry,
            done,
        }
    }

    // generate an output line string with a checkbox
    pub fn file_line(&self) -> String {
        // tick or untick checkbox by status
        let checkbox_status = if self.done { "[*] " } else { "[ ] " };
        format!("{}{}\n", checkbox_status, self.todo_entry,)
    }

    // manage how lines are formatted for the command line
    pub fn list_line(&self, number: usize) -> String {
        // check if current task is completed or not
        let todo_entry = if self.done {
            // DONE:    task is completed -> so print it with a strikethrough
            self.todo_entry.strikethrough().to_string()
        } else {
            // NOT DONE:    task is NOT completed -> print as is
            self.todo_entry.clone()
        };
        format!("{number} {todo_entry}\n");
    }

    pub fn read_line(line: &String) -> Self {
        // interpret the checkbox status from lines to deduce status
        let done = &line[..4] == "[*] "; // ticked true -> done true
        let todo_entry = (&line[4..]).to_string();  // remainder of line is the item
        Self {
            todo_entry, 
            done,
        }
    }

    // return the ToDo item without formatting
    pub fn raw_line(&self) -> String {
        format!("{}\n", self.todo_entry)
    }
}

pub struct Todo {
    pub todo: Vec<String>,
    pub todo_path: String,
    pub todo_bak: String,
    pub no_backup: bool,
}

// TODO: continue

