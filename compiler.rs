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
    // cur_statement.push(String::from(""));
    // params.push(Vec::new());
    print!("({})\n", params.len());
    read_file(&mut contents)?;
    for c in contents.chars() {
        do_bytecode(c,&mut depth, &mut params, &mut cur_statement, &mut macros);
        // print!("{}", c);
    }
    Ok(())
}



fn do_bytecode(c: char, depth: &mut usize, params: &mut Vec< Vec<String>>, cur_statement: &mut Vec<String>, macros: &mut Vec<String>) {
    print!("{}", c);
    if c == '(' {
        print!("\r\t\t\t\t");
        for param in &mut *params {
                print!("\t{:?}", param);
        }
        print!("\r\t\t\tlen: {}\tdepth: {}\n", params.len(), depth);
        *depth += 1;
        params.push(Vec::new());
    }
    else if c == ')' {
        *depth -= 1;
        eval_param(params.last());
        params.pop();
        print!("\r\t\t\t\t");
        for param in &mut *params {
                print!("\t{:?}", param);
        }
        print!("\r\t\t\tlen: {}\tdepth: {}\n", params.len(), depth);
    }
    else {
        
    }
}

fn eval_param (param: &mut Option<Vec<String>>) {
    
}
// {

//     // ensuring that the read type is set in the first place
//     if cur_statement.len() >= *depth && false {
//         cur_statement.push(String::from(""));
//         // params.push(Vec::new());
//     }

//     if c == '(' {
//         // increase depth due to paren
//         *depth += 1;
//         //create new set of params
//         params.push(Vec::new());
//         print!("{}",depth);
//         cur_statement[*depth].push('(');
//     }
//     else if c == ')' {
//         //add param to prev
//         let tmp = String::from(&cur_statement[*depth]);
//         params[*depth].push(tmp.clone());
//         cur_statement[*depth-1] += &tmp;
//         // print!("| len: {}\tdepth: {} |", params/*[*depth]*/.len(), depth);
//         print!("{:?}",params);
//         *depth -= 1;
//     }
// }

fn read_file(contents: &mut String) -> std::io::Result<()> {
    let mut file = File::open(FILENAME)?;
    file.read_to_string(contents)?;
    Ok(())
}
