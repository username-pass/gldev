use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const FILENAME: &str = "test.gl";

fn main() -> std::io::Result<()> {
    let mut defs = Vec::new();
    //definitions defined as follows:
    // <funct-name> <default> <next-param> <next-param>
    load_defs(&mut defs);
    // actual code, load it into memory
    let mut contents = Vec::new();
    let mut parens = Vec::new();
    // contents are as follows:
    // (char, depth, )
    read_file(&mut contents, &mut parens)?;

    for (i, (c, token_type, depth, delta)) in contents.iter().enumerate() {
        print!(
            "c: {},\ti:{}\tt: {},\tD: {},\td: {}\n",
            c, i, token_type, depth, delta
        );
    }

    // fill_in_parens(&mut contents);

    // for (c, i) in contents.enumerate() {
    // do_bytecode(c, i, &mut contents, &mut defs, &mut depth);
    // print!("{}", c);
    // }
    Ok(())
}

fn load_defs(defs: &mut Vec<String>) {
    //while loop definition
    defs.push("wnxnppeswhile\n npeesend\n ".to_string());
    //init command definition
    defs.push("Nxnsinit pe".to_string());
    //comments
    defs.push("cnxn".to_string());
}

/* fn fill_in_parens(
    parens: &mut Vec<(usize, usize)>
) {
    let mut delta = 0;
    let mut depth = 0;
    let mut has_started = false;
    // Look into this:
    // may need a second vec to store the positions by a value?!?
    // Stuff is slightly more complicated than I thought. We'll see
    // though

    let mut i = 0;
    for (c, token_type, start, end) in contents.iter() {
        if *c == '(' {
            contents[depth + delta].1 = i;
            depth += 1;
            if has_started {
                delta += 1;
            }
        } else if *c == ')' {
            depth -= 1;
            delta += 1;
        }
    }
} */

fn do_bytecode(
    c: *mut (char, usize, usize),
    depth: &mut usize,
    params: &mut Vec<Vec<String>>,
    cur_statement: &mut Vec<String>,
    macros: &mut Vec<String>,
) {
}

fn do_bytecode_old(
    c: char,
    depth: &mut usize,
    params: &mut Vec<Vec<String>>,
    cur_statement: &mut Vec<String>,
    macros: &mut Vec<String>,
) {
    print!("{}", c);
    if c == '(' {
        print!("\r\t\t\t\t");
        for param in &mut *params {
            print!("\t{:?}", param);
        }
        print!("\r\t\t\tlen: {}\tdepth: {}\n", params.len(), depth);
        *depth += 1;
        params.push(Vec::new());
    } else if c == ')' {
        *depth -= 1;
        // eval_param(params.last());
        params.pop();
        print!("\r\t\t\t\t");
        for param in &mut *params {
            print!("\t{:?}", param);
        }
        print!("\r\t\t\tlen: {}\tdepth: {}\n", params.len(), depth);
    } else {
    }
}

fn eval_param(param: &mut Option<Vec<String>>) {}
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

fn read_file(
    contents: &mut Vec<(char, usize, usize, usize)>,
    parens: &mut Vec<(usize, usize)>,
) -> std::io::Result<()> {
    // contents: char, type, depth, delta
    // parens: start, end
    print!("reading file!\n");
    let mut tmp = String::new();
    let mut file = File::open(FILENAME)?;
    file.read_to_string(&mut tmp)?;
    let mut depth = 0;
    let mut delta = 0;
    let mut token_type = 3;
    let mut last_type = 3;
    // 0 = solo param
    // 1 = command
    // 2 = deliminator between chars (whitespace)
    // 3 = open paren
    // 4 = close param
    print!("tmp: {}\n", tmp);
    for (i, c) in tmp.chars().enumerate() {
        last_type = token_type;
        token_type = 0;
        // update deltas and stuff for parens

        if c == '(' {
            // increment depth, move down
            depth += 1;
            token_type = 3;
            print!(
                "opening at len: i:{}\tlen:{}\tdepth+delta:{}\n",
                i,
                parens.len(),
                (depth + delta)
            );
            // add to parens list
            parens.push((i, i));
        } else if c == ')' {
            // move this to next iteration to keep close paren inside
            // the depth, next depth only applies on next iteration
            /* depth -= 1;
            delta += 1; */
            token_type = 4;
            // update parens param
            parens[depth + delta - 1].1 = i;
            print!(
                "closing at len: i:{}\tlen:{}\tdepth+delta:{}\n",
                i,
                parens.len(),
                (depth + delta)
            );
        } else if c == ' ' || c == '\t' || c == '\n' {
            // whitespace
            token_type = 2;
        } else if last_type == 3 {
            // command
            // started with open paren
            token_type = 1;
        } else if last_type == 2 {
            // if last type was a deliminator
            // it must be a solo param
            // if it's going into a param, increase depth
            print!("entering param\n");
            parens.push((i, i));
            depth += 1;
            token_type = 0;
        } else {
            // base case, defaulting to param
            print!("base case, char: {}\n", c);
            token_type = 0;
        }

        if last_type == 0 && token_type != 0 {
            //changing out of param to not param
            // add to delta
            parens[depth + delta - 1].1 = i - 1;
            print!("exiting param\n");
            depth -= 1;
            delta += 1;
        } else if last_type == 4 {
            // now increment depth
            depth -= 1;
            delta += 1;
        }
        // check for single tokens
        print!(
            "c: {}\tlast: {}\ttype: {}\tdepth: {}\tdelta: {}\n",
            c, last_type, token_type, depth, delta
        );
        contents.push((c, token_type, depth, delta));
    }
    Ok(())
}
