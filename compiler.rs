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
    // contents are as follows:
    // (char, type, depth, delta)
    let mut parens = Vec::new();
    // parens are as follows
    // (start, end)
    // index = depth + delta - 1 (to account for zero indexing)
    read_file(&mut contents, &mut parens)?;
    do_bytecode(&mut contents, &mut parens, &mut defs);
    // debug
    // output should match hand-written code
    for (i, (c, token_type, depth, delta)) in contents.iter().enumerate() {
        print!(
            "c: {},\ti:{}\tt: {},\tD: {},\td: {}\n",
            c, i, token_type, depth, delta
        );
    }
    for (i, (start, end, marker)) in parens.iter().enumerate() {
        print!("idx: {}\t({},\t{})\t{}\n", i, start, end, marker);
    }
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

fn do_bytecode(
    contents: &mut Vec<(char, usize, usize, usize)>,
    parens: &mut Vec<(usize, usize, bool)>,
    defs: &mut Vec<String>,
) {
}

fn read_file(
    contents: &mut Vec<(char, usize, usize, usize)>,
    parens: &mut Vec<(usize, usize, bool)>,
) -> std::io::Result<()> {
    // contents: char, type, depth, delta
    // parens: start, end
    print!("reading file!\n");
    let mut tmp = String::new();
    let mut file = File::open(FILENAME)?;
    // in the bootstrapped code, this will be implemented by just reading all
    // characters to the array
    file.read_to_string(&mut tmp)?;
    //Defining token types
    let SOLO_PARAM = 0;
    let COMMAND = 1;
    let DELIMINATOR = 2;
    let OPEN_PAREN = 3;
    let CLOSE_PAREN = 4;
    //
    let mut depth = 0;
    let mut delta = 0;
    let mut token_type = OPEN_PAREN;
    let mut last_type = OPEN_PAREN;
    print!("code:\n{}\n", tmp);
    for (i, c) in tmp.chars().enumerate() {
        last_type = token_type;
        token_type = SOLO_PARAM;
        // update deltas and stuff for parens

        if last_type == SOLO_PARAM && token_type != SOLO_PARAM {
            //changing out of param to not param
            // add to delta
            print!("exiting param\n");
            // WATCH OUT, ITER MIGHT BE WRONG, YOU MAY NEED TO DO
            // A len - i INSTEAD
            for (j_rev, (start, end, marker)) in parens.iter().rev().enumerate() {
                let j = parens.len() - j_rev - 1;
                print!("backiter, j = {}\n", j);
                print!("j: {}\tdepth: {}\n", j, depth);
                if *marker {
                    continue;
                }
                // adding 1 to account for zero indexing
                // - may remove at some point if it makes things cleaner
                delta = 1 + j - depth;
                break;
            }
            parens[depth + delta - 1].1 = i - 1;
            parens[depth + delta - 1].2 = true;
            depth -= 1;
        } else if last_type == 4 {
            // now increment depth
            depth -= 1;
            delta += 1;
        }

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
            parens.push((i, 0, false));
            delta = parens.len() - depth;
            assert!(parens.len() == (depth + delta));
        } else if c == ')' {
            // move this to next iteration to keep close paren inside
            // the depth, next depth only applies on next iteration
            /* depth -= 1;
            delta += 1; */
            token_type = 4;

            // WATCH OUT, ITER MIGHT BE WRONG, YOU MAY NEED TO DO
            // A len - i INSTEAD
            //find delta with markers
            for (j_rev, (start, end, marker)) in parens.iter().rev().enumerate() {
                let j = parens.len() - j_rev - 1;
                if *marker {
                    continue;
                }
                // adding 1 to account for zero indexing
                // - may remove at some point if it makes things cleaner
                delta = 1 + j - depth;
                print!("j: {}\tdepth: {}\tdelta: {}\n", j, depth, delta);
                break;
            }

            // update parens param
            // print!("depth+delta-1 = {:?}\n", parens[depth + delta - 1]);
            parens[depth + delta - 1].1 = i;
            parens[depth + delta - 1].2 = true;
            /* print!(
                "\t\tparen item:{:?}\tchanged:{}\n",
                parens[depth + delta - 1],
                parens[depth + delta - 1].1
            ); */
            /* print!(
                "closing at len: i:{}\tlen:{}\tdepth+delta:{}\n",
                i,
                parens.len(),
                (depth + delta)
            ); */
        } else if c == ' ' || c == '\t' || c == '\n' {
            // whitespace
            token_type = DELIMINATOR;
        } else if last_type == OPEN_PAREN || last_type == COMMAND {
            // command
            // started with open paren
            token_type = COMMAND;
        } else if last_type == DELIMINATOR {
            // if last type was a deliminator
            // it must be a solo param
            // if it's going into a param, increase depth
            // print!("entering param\n");
            parens.push((i, i, true));
            depth += 1;
            delta = parens.len() - depth;
            token_type = SOLO_PARAM;
        } else if last_type == SOLO_PARAM {
            // assuming that it is a mutli-char param
            // base case, defaulting to param
            print!("base case, char: {}\n", c);
            depth += 1;
            print!("depth: {}\tlen: {}\n", depth, parens.len());
            print!("delta: {}\n", parens.len() - depth);

            delta = parens.len() - depth;
            // print!("==LAST===  {}", parens.last().expect((i - 1, i - 1, true)));
            let len = parens.len();

            parens[len - 1].1 = i;

            token_type = SOLO_PARAM;
        }

        // check for single tokens
        /* print!(
            "c: {}\tlast: {}\ttype: {}\tdepth: {}\tdelta: {}\n",
            c, last_type, token_type, depth, delta
        ); */
        contents.push((c, token_type, depth, delta));
        if token_type == SOLO_PARAM && (last_type == DELIMINATOR || last_type == SOLO_PARAM) {
            depth -= 1;
        }
    }
    Ok(())
}
