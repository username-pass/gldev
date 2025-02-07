use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

const FILENAME: &str = "source.gl";

fn main() -> std::io::Result<()> {
    //file reading stuff
    let mut file = File::open(FILENAME)?;
    let mut contents = String::new();

    //important definitions
    
    // create macros map
    let mut macros = Vec::new();
    let mut bytecode_stack = Vec::new();
    bytecode_stack.push("testing");

    macros.push(new_macromap());

    // depth is how many layers deep in the nesting we are
    let mut depth = 0;

    // 0 = command
    // 1 = param
    let mut read_type = 0;

    let mut cur_argument = String::from("");
    
    file.read_to_string(&mut contents)?;
    // assert_eq!(contents, "Hello, world!\n");
    for c in contents.chars() {

        // print!("{}",c);

        //ignore whitespace
        if read_type == 0 && (c == ' ' || c == '\t' || c == '\n') {
            continue;
        }
        else if c == '(' {
            depth += 1;
            read_type = 0;
            print!("depth: {}\n",depth);
        }
        else if c == ')' {
            depth -= 1;
            read_type = 0;
            if cur_argument.chars().count() > 0 {
                print!("evaluating arg '{}'\n", cur_argument);
                cur_argument = String::from("");
            }
            // print!("moving up in depth\n")
        }
        // now evaluate parameters and functions
        else if read_type == 0 {
            read_type = 1;
            do_bytecode(c, depth, macros.clone(), &mut bytecode_stack)
        }
        else if read_type == 1 {
            cur_argument.push(c);
        }
    }
    Ok(())
}

fn new_macromap() -> std::collections::HashMap<String, String> {
    return HashMap::new();
}
fn do_bytecode(c: char, depth: i32, macros: Vec<std::collections::HashMap<String, String>>, bytecode_stack: &mut Vec<&str>) {
    print!("running command {}\n", c);
    bytecode_stack.push("test")
}
