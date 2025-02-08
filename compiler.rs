use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

const FILENAME: &str = "source.gl";

fn main() -> std::io::Result<()> {

    let mut params = Vec::new();
    let mut macros = Vec::new();
    let mut depth = 0;
    //read types:
    // 0 = command (trying to find command)
    // 1 = param 
    let mut cur_statement = Vec::new();
    let mut contents = String::new();

    // push default
    cur_statement.push(String::from(""));
    params.push(Vec::new());
    
    read_file(&mut contents)?;
    for c in contents.chars() {
        do_bytecode(c,&mut depth, &mut params, &mut cur_statement, &mut macros);
        print!("{}", c);
    }
    Ok(())
}
fn do_bytecode(c: char, depth: &mut usize, params: &mut Vec< Vec<String>>, cur_statement: &mut Vec<String>, macros: &mut Vec<String>) {

    // ensuring that the read type is set in the first place
    if cur_statement.len() >= *depth {
        cur_statement.push(String::from(""));
        params.push(Vec::new());
    }

    if c == '(' {
        // increase depth due to paren
        *depth += 1;
        //create new set of params
        params.push(Vec::new());
        print!("{}",depth);
        cur_statement[*depth].push('(');
    }
    else if c == ')' {
        //add param to prev
        let tmp = String::from(&cur_statement[*depth]);
        params[*depth-1].push(tmp.clone());
        print!("| len: {}\tdepth: {} |", params[*depth].len(), depth);
        cur_statement[*depth-1] += &tmp;
        *depth -= 1;
    }
}

fn read_file(contents: &mut String) -> std::io::Result<()> {
    let mut file = File::open(FILENAME)?;
    file.read_to_string(contents)?;
    Ok(())
}
