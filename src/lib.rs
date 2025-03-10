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
use std::io::{self, BufReader, BufWriter, Stdout, Write};
use std::path::Path;
use std::{env,process};

// outline a struct, cf "object"(-ish)
pub struct Entry {
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
        format!("{number} {todo_entry}\n")
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
impl Todo {
    pub fn new() -> Result<Self, String> {
        // define a path to store the list at
        let todo_path: String = match env::var("TODO_PATH") {
            Ok(t) => t,
            Err(_) => {
                let home = env::var("HOME").unwrap();
                // look for legacy TODO file path
                let legacy_todo = format!("{}/TODO", &home);
                match Path::new(&legacy_todo).exists() {
                    true => legacy_todo,
                    false => format!("{}/.todo", &home),
                }
            }
        };

        // define a backup path
        let todo_bak : String = match env::var("TODO_BAK_DIR") {
            Ok(t) => t,
            Err(_) => String::from("/tmp/todo.bak"),
        };

        let no_backup = env::var("TODO_NOBACKUP").is_ok();

        let err_msg: String = format!("Couldn't open the todofile at {}", &todo_path);
        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&todo_path)
            .expect(&err_msg);

        // create a new buffer reader to generate and modify todo list file
        let mut buf_reader = BufReader::new(&todofile);
        // create an empty String to be filled with ToDos
        let mut contents = String::new();
        // load pre-existing data into contents String
        buf_reader.read_to_string(&mut contents).unwrap();

        // split contents of TODO file into a todo Vector
        let todo = contents.lines().map(str::to_string).collect();

        Ok(Self {
            todo,
            todo_path,
            todo_bak,
            no_backup,
        })
    }

    // print every saved todo
    pub fn list(&self) {
        let stdout: Stdout = io::stdout();
        // set up buffered writer for stdout stream
        let mut writer = BufWriter::new(stdout);
        let mut data = String::new();
        // foreach task in TODO file
        for (number, task) in self.todo.iter().enumerate() {
            let entry = Entry::read_line(task);
            let number = number+1; // 0-indexed language, 1-indexed ToDo-list
            let line = entry.list_line(number);
            data.push_str(&line);
        }
        writer
            .write_all(data.as_bytes())
            .expect("Failed to write data to stdout.");
    }

    // print only already done OR still todo tasks as raw strings, useful for scripting
    pub fn raw(&self, arg: &[String]) {
        if arg.len() > 1 {
            eprintln!("ToDo raw takes only 1 argument; not {}", arg.len());
        } else if arg.is_empty() {
            eprintln!("ToDo raw takes 1 argument (done/todo), but none was given");
        } else {
            let stdout = io::stdout();
            // set up buffered writer for stdout stream
            let mut writer = BufWriter::new(stdout);
            let mut data = String::new();
            let arg = &arg[0];
            // foreach task in TODO file
            for task in self.todo.iter() {
                let entry = Entry::read_line(task);
                if entry.done && arg == "done" {
                    data = entry.raw_line(); // list only "already done" tasks
                } else if !entry.done && arg == "todo" {
                    data = entry.raw_line(); // list only "still todo" tasks
                } // else: leave data as blank string
                
                let err_msg = format!("Failed to write task {} to stdout", &data);
                writer
                    .write_all(data.as_bytes())
                    .expect(&err_msg);
            }
        }
    }

    // add new todos; supports bulk add -> n_args >= 1
    pub fn add (&self, args:&[String]) {
        if args.is_empty() {
            eprintln!("todo add takes at least 1 argument");
            process::exit(1);            
        }
        // open todo file with permissions to: 
        let err_msg = format!("Couldn't open the todo file at {}", &self.todo_path);
        let todofile = OpenOptions::new()
            .create(true)   // a) create the file if !exists
            .append(true)   // b) append lines to the file
            .open(&self.todo_path)
            .expect(&err_msg);

        let mut buffer = BufWriter::new(todofile);
        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }
            // append new task(s) to the file
            let entry = Entry::new(arg.to_string(), false); 
            let line= entry.file_line();
            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
    }

    // remove tasks by item number on todo list
    pub fn remove(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo rm takes at least 1 argument");
            process::exit(1);
        }
        // open todo file with permissions to:
        let err_msg = format!("Couldn't open the todo file at {}", &self.todo_path);
        let todofile = OpenOptions::new()
            .write(true)    // write
            .truncate(true) // truncate
            .open(&self.todo_path)
            .expect(&err_msg);
        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in self.todo.iter().enumerate() {
            if args.contains(&(pos+1).to_string()) {
                continue;   // if index+1 == list slot to remove
            }
            let line = format!("{}\n", line);
            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
    }

    fn remove_file(&self) {
        match fs::remove_file(&self.todo_path) {
            Ok(_) => {},
            Err(e) => {
                eprint!("Error while clearing todo file: {}", e)
            }
        };
    }

    // clear/reset todo-list by removing todo file (incl. tmp backup)
    pub fn reset(&self) {
        if !self.no_backup {
            match fs::copy(&self.todo_path, &self.todo_bak) {
                Ok(_) => self.remove_file(),
                Err(e) => {
                    eprint!("Couldn't backup todo file -> no action taken; reason: {}", e);
                }
            }
        } else {
            self.remove_file();
        }
    }

    // attempt to restore file from backup, if exists
    pub fn restore(&self) {
        let err_msg = format!("unable to restore backup file from {}", &self.todo_bak);
        fs::copy(&self.todo_bak, &self.todo_path)
            .expect(&err_msg);
    }

    // sort tasks by self.done?, optionally also alphabetical
    pub fn sort(&self, arg: &[String]) {
        assert!(!(arg.len() >= 1), "todo sort takes either no arguments, or the single special argument '--alphabetical'");
        let sort_alphabetical : bool = match arg.len() == 1 {
            true => match arg[0].eq("--alphabetical") {
                true => true,
                false => {
                    eprint!("todo sort takes either no arguments, or the special argument '--alphabetical'");
                    process::exit(1);
                }
            }
            false => false,
        };

        // create a mutable clone of todo item vector to be able to sort alphabetically
        let mut todo_cp : Vec<String> = self.todo.clone();
        if sort_alphabetical {
            todo_cp.sort();
        }

        let new_todo: String;
        let mut todo = String::new();
        let mut done = String::new();
        // split list into two sub-lists -> stable
        for line in todo_cp.iter() {
            let entry = Entry::read_line(line);
            if entry.done {
                let line = format!("{}\n", line);
                done.push_str(&line);
            } else {
                let line = format!("{}\n", line);
                todo.push_str(&line);
            }
        }

        new_todo = format!("{}{}", &todo, &done);
        // open the todo file with permission to:
        let mut todofile = OpenOptions::new()
            .write(true)    // a) write
            .truncate(true) // b) truncate
            .open(&self.todo_path)
            .expect("Error while trying to save the todo file");

        // write contents of a new todo list to todo file
        todofile
            .write_all(new_todo.as_bytes())
            .expect("Error while trying to save the todofile")
    }

    pub fn done(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo done takes at least 1 argument");
            process::exit(1);
        }

        // open the todo file with overwrite permission
        let todofile = OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");
        let mut buffer = BufWriter::new(todofile);
        let mut data = String::new();

        for (pos, line) in self.todo.iter().enumerate() {
            let mut entry = Entry::read_line(line);
            let line = if args.contains(&(pos+1).to_string()) {
                // flip the done bool if item index is in arg list
                entry.done = !entry.done;
                entry.file_line()
            } else {
                format!("{}\n", line)
            };
            data.push_str(&line);
        }
        buffer
            .write_all(data.as_bytes())
            .expect("unable to write data")
    }

    pub fn edit(&self, args:&[String]) {
        if args.is_empty() || args.len() != 2 {
            eprintln!("todo edit takes exactly 2 arguments");
            process::exit(1);
        }
        // open the todo file with overwrite permission
        let todofile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todo file");
        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in self.todo.iter().enumerate() {
            let line = if args[0].contains(&(pos+1).to_string()) {
                // if item index matches, overwrite original item description with new str from argument
                let mut entry = Entry::read_line(line);
                entry.todo_entry = args[1].clone();
                entry.file_line()
            } else {
                format!("{}\n", line)
            };
            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data")
        }
    }
}

const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: todo add \"buy carrots\"
    - edit [INDEX] [EDITED TASK/s]
        edits an existing task/s
        Example: todo edit 1 banana
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX]
        removes a task
        Example: todo rm 4
    - reset
        deletes all tasks
    - restore 
        restore recent backup after reset
    - sort
        sorts completed and uncompleted tasks
        optional flags: --alphabetical : sort items alphabetical before sorting by completion
        Example: todo sort --alphabetical
    - raw [todo/done]
        prints nothing but done/incompleted tasks in plain text, useful for scripting
        Example: todo raw done
";

pub fn help() {
    // print nicely readable HELP statement
    println!("{}", TODO_HELP);
}