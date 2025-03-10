/**
 * This file is mostly a 1:1 transcript/copy of the code published by [sioodmy](https://github.com/sioodmy/todo) under a GPL-3.0 license.
 * THANK YOU!
 * 
 * Slightly edited and commented by [timmimim](https://github.com/timmimim).
 * This file has been created for training purposes.
 */

use std::env;
use practice_todo_list::{help, Todo};

fn main() {
    let todo = Todo::new().expect("Couldn't create the todo instance."); 

    let args: Vec<String> = env::args().collect();

    // if more than one argument is given, these are to be handled as commands and (potentially) new ToDo list items
    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list"  =>  todo.list(),
            "add"   =>  todo.add(&args[2..]),
            "rm"    =>  todo.remove(&args[2..]),
            "done"  =>  todo.done(&args[2..]),
            "raw"   =>  todo.raw(&args[2..]),
            "edit"  =>  todo.edit(&args[2..]),
            "sort"  =>  todo.sort(&args[2..]),
            "reset" =>  todo.reset(),
            "restore" => todo.restore(),
            "help" | "--help" | "-h" | "h" | _ => help(),
        }
    } else{
        todo.list();
    }
}