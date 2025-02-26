use std::fs::File;
use std::io::prelude::*;

const FILENAME: &str = "test.gl";
const SOLO_PARAM: u8 = 0;
const COMMAND: u8 = 1;
const DELIMINATOR: u8 = 2;
const OPEN_PAREN: u8 = 3;
const CLOSE_PAREN: u8 = 4;
const DEBUG: bool = true;

fn main() -> std::io::Result<()> {
    let mut defs = String::from("");
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
    let mut commands = Vec::new();
    // the commands used in the code, stored in a vec so that it can be referenced later
    commands.push((String::from(""), String::from("pwq")));
    // push a dummy param for all empty ones. just says to eval

    read_file(&mut contents, &mut parens, &mut commands, &mut defs)?;
    // debug
    // output should match hand-written code
    if DEBUG {
        for (i, (c, token_type, depth, delta)) in contents.iter().enumerate() {
            print!(
                "c: {},\ti:{}\tt: {},\tD: {},\td: {}\n",
                c, i, token_type, depth, delta
            );
        }
        for (i, (start, end, cmd_loc, marker)) in parens.iter().enumerate() {
            print!(
                "idx: {}\t({},\t{})\tcmd: {}\t{}\n",
                i, start, end, cmd_loc, marker
            );
        }
    }
    do_bytecode(&mut contents, &mut parens, commands, defs);
    Ok(())
}

fn load_defs(defs: &mut String) {
    //while loop definition
    defs.push_str("wnxnppeswhile\n npeesend\n q");
    //init command definition
    defs.push_str("Nnxnsinit pws\n q");
    //comments
    defs.push_str("cnxq");
    //temp thing to just evaluate things
    defs.push_str("$npwq");
    //temp thing to just evaluate things
    defs.push_str("#npwq");
    //temp thing to just evaluate things
    defs.push_str(">npwq");
    //temp thing to just evaluate things
    defs.push_str("-npwq");
    print!("defs: {}", defs);
}

fn do_bytecode(
    contents: &mut Vec<(char, u8, usize, usize)>,
    // contents: (char, type, depth, delta)
    parens: &mut Vec<(usize, usize, usize, bool)>,
    // parens: (start, end, command_loc, marker)
    commands: Vec<(String, String)>,
    // commands: (name, def)
    defs: String,
) {
    // this is the state at a given param
    let mut state: Vec<(usize, usize, usize, usize)> = Vec::new();
    // state holds the state like this:
    // each depth has its own value in the following format:
    // (definition_idx, cur_idx, cmd_loc, delta)
    let mut global_idx = 0;
    let mut last_type = CLOSE_PAREN;
    let mut command_stack = Vec::new();
    let SEARCHING_CMD = 0;
    let WRITING = 1;
    let SEARCHING_PARAM = 2;
    // only temporary
    let EVALUATING = 3;
    let mut mode = SEARCHING_CMD;
    let mut cur_depth = 0;
    let mut output = String::from("");
    // this should work as each item in the loop does one thing. There shouldn't be
    // too many nested layers.
    // The exception is writing a string, where it just repeatedly writes the string
    // that might also change
    // the idea is that you change modes, and each mode does one different thing
    // so write might write the current char, and search cmd might just move to the
    // next cmd
    loop {
        if global_idx >= contents.len() {
            break;
        }
        if mode == SEARCHING_CMD {
            let (c, token_type, depth, delta) = contents[global_idx];
            cur_depth = depth;
            if token_type == COMMAND {
                loop {
                    if state.len() >= depth + delta - 1 {
                        break;
                    }
                    // junk items, should never be used, in theory
                    state.push((0, 0, 0, 0));
                }
                // found command, need to start evaluating
                mode = SEARCHING_PARAM;
            } else {
                // keep searching
                global_idx += 1;
            }
        } else if mode == SEARCHING_PARAM {
            let (c, token_type, depth, delta) = contents[global_idx];
            if token_type == OPEN_PAREN {
                // found param, need to start evaluating previous index
                let (c_prev, token_type_prev, depth_prev, delta_prev) = contents[global_idx - 1];
                let (start, end, cmd_loc, _) = parens[depth_prev + delta_prev - 1];
                let (cmd, def) = commands[cmd_loc];
                // get cmd def, and try to find what to do next
                let (def_idx, _, _, _) = &mut state[depth + delta - 1];
                let cur_def_cmd = def.chars().nth(*def_idx).unwrap();
                if cur_def_cmd == 'x' {
                    // no-op, do nothing
                } else if cur_def_cmd == 's' {
                    //write value to string
                } else if cur_def_cmd == 'p' {
                    // push value to stack
                } else if cur_def_cmd == 'e' {
                    // start evaluating again
                } else if cur_def_cmd == 'w' {
                    // start writing the current param to output
                }
            } else {
                global_idx += 1;
            }
        } else if mode == EVALUATING {
            // evaluating the parameters
            let (c, token_type, depth, delta) = contents[global_idx];
            let (start, end, cmd_loc, _) = &mut parens[depth + delta - 1];
            let &(cmd, def) = &commands[*cmd_loc];
            let (definition_idx, cur_idx, cmd_loc, delta) = &state[depth + delta - 1];
            let cur_def_cmd = def.chars().nth(*definition_idx).unwrap();
            if cur_def_cmd == 'p' {
                //push value to stack
                command_stack.push((start, end))
            } else if cur_def_cmd == 'w' {
                loop {}
            }

            *definition_idx += 1;
        }
    }
}
// OLD
/* //previous items
if global_idx > 0 {
    let (c_prev, token_type_prev, depth_prev, delta_prev) = (contents[global_idx - 1]);
}
// current items
let (c, token_type, depth, delta) = contents[global_idx];
if token_type == COMMAND {
    let (start, end, cmd_loc, _) = parens[depth + delta - 1];
    // add until the state matches the depth + delta (paren loc)
    loop {
        if state.len() >= depth + delta - 1 {
            break;
        }
        state.push((0, 0, 0));
    }
    // push current state
    state.push((delta, global_idx, 0, cmd_loc));
    // command, do stuff
    print!("evaluating command {}\n", cmd);
    print!(
        "depth + delta: {}\tstate.len(): {}\n",
        depth + delta,
        state.len()
    );
} else if token_type == OPEN_PAREN {
    // beginning of a param, check current state and deal with it
    let (delta, cur_idx, def_idx, cmd_loc) = &state[depth_prev + delta_prev - 1];
    let (cmd, def) = &commands[depth_prev + delta_prev - 1];

} */

/* fn do_bytecode_less_old(
    contents: &mut Vec<(char, u8, usize, usize)>,
    parens: &mut Vec<(usize, usize, usize, bool)>,
    defs: String,
) {
    // curren command for every depth (so it can be nested)
    // (cmd name, cmd def)
    let mut cur_command = Vec::new();
    // the stack of commands for bytecode generation
    let mut command_stack = Vec::new();
    let mut last_type = OPEN_PAREN;
    let mut cmd_pointer = 0;

    //loop through the code
    loop {
        if cmd_pointer >= contents.len() {
            break;
        }
        let (c, token_type, depth, delta) = contents[cmd_pointer];
        print!("token type: {}\tcmd_pointer: {}\n", token_type, cmd_pointer);
        if token_type == COMMAND {
            // it's a command, need to find the definition
            // make the command at the depth exist
            print!("depth: \t{}, len: {}\n", depth, cur_command.len());
            loop {
                if cur_command.len() > depth {
                    break;
                } else {
                    cur_command.push((String::from(""), String::from("")));
                }
            }
            // reset current command
            cur_command[depth] = String::from(c);

            // get the rest of the command
            loop {
                cmd_pointer += 1;
                let (c, token_type, depth, delta) = contents[cmd_pointer];
                if token_type == COMMAND {
                    // add the rest of the command
                    cur_command[depth].push((c, String::from("")));
                } else {
                    break;
                }
            }
            print!("command: {:?}\n", cur_command[depth]);
            // found command
            // now just need to find parameters
            // step one is find corresponding parens
            let (start, end, _, _) = parens[depth + delta - 1];

            let (cmd, def) = find_def(defs.clone(), cur_command[depth].1.clone());
            cur_command[depth] = (cmd, depth);
            print!("command: {}\tdef: {}\n", cmd, def);
            print!("start: {}\tend: {}\n", start, end);

            //now just need to apply the definitions

            // steps to do so:
            // 1) go until you reach an open paren
            // 2) read and execute all commands for that section
            // 3) skip to end of open paren
            // 4) repeat

            let mut def_cmd_pointer = 0;
            let mut def_is_escaped = false;
            loop {
                if cmd_pointer >= contents.len() {
                    break;
                }
                let (c, token_type, depth, delta) = contents[cmd_pointer];
                if token_type == OPEN_PAREN {
                    //need to start doing stuff
                    loop {
                        // break if it's at an 'n', and not escaped, or out of scope
                        if (def[def_cmd_pointer] == 'n' && !def_is_escaped)
                            || def_cmd_pointer >= def.len()
                        {
                            break;
                        }
                        let bytecode_cmd = def[def_cmd_pointer];
                        if bytecode_cmd == 'p' {
                            // pushes param to stack
                            command_stack.push(parens[depth + delta - 1]);
                        } else if bytecode_cmd == 'e' {
                            // pops value from stack
                            // then writes the value
                            let param = command_stack.last();
                        }
                    }
                }

                cmd_pointer += 1;
            }
        } else {
            cmd_pointer += 1;
        }

        last_type = token_type;
    }
} */

fn find_def(defs: String, searchfor: String) -> (String, String) {
    // now find definition
    let mut def_idx: usize = 0; // the position in the definition vec
    let mut def_start = 0; // the start of the current definition
    let mut cmd = String::from(""); // the actual command
    let mut def = String::from("pwq"); // the full definition
    let mut is_escaped = false;
    let mut looking_for_command = true;
    loop {
        if def_idx >= defs.len() {
            break;
        }
        let cur_char = defs.chars().nth(def_idx).unwrap();
        // escape characters
        if cur_char == '\\' && !is_escaped {
            // make it escaped
            is_escaped = true;
            // increment idx since it's going to skip everything
            def_idx += 1;
            // skip, there's nothing else to do
            continue;
        }

        // command specific chars
        if cur_char == 'q' && !is_escaped {
            // push the last char to def, just in case
            def.push('q');

            //     print!("reached end, checking...\n");
            //     print!(
            //         "is equal: {}\tsearch_for: {}\tcmd: {}\tdef: {}\n",
            //         searchfor == cmd,
            //         searchfor,
            //         cmd,
            //         def
            //     );
            // check if it's the correct definition
            if searchfor == cmd {
                // it's the correct command, exit loop, all is well
                //         print!("found correct def! {}\n", cmd);
                return (cmd, def);
                // break;
            }

            // end of def, reset
            // set start to next char
            def_start = def_idx + 1;
            // make it look for the command
            looking_for_command = true;
            // set def to blank
            def = String::from("");
            cmd = String::from("");
        }
        // next param
        else if cur_char == 'n' && looking_for_command && !is_escaped {
            looking_for_command = false;
        } else if looking_for_command {
            //     // print!("added to def: {}\n", cur_char);
            // if we're looking for a command, add to command
            cmd.push(cur_char);
        } else {
            //     // print!("added to cmd {}\n", cur_char);
            // otherwise, just add to general definition
            def.push(cur_char);
        }

        // add char when not command

        // // print!("{}", cur_char);

        def_idx += 1;
    }
    return (cmd, def);
}

/* fn do_bytecode_old(
    contents: &mut Vec<(char, u8, usize, usize)>,
    parens: &mut Vec<(usize, usize, usize, bool)>,
    defs: String,
) {
    let mut cur_cmd = String::from("");
    let mut last_type = OPEN_PAREN;
    let mut i = 0;

    // for (i, (c, token_type, depth, delta)) in contents.iter().enumerate() {
    loop {
        let (c, token_type, depth, delta) = contents[i];
        // print!("contents: {:?}\tc: {}\n", contents[i], c);

        // default to comment
        let mut def = "cnxq".to_string();
        if token_type == COMMAND {
            // check if it's a command. If it is, create the command string.
            // In the bootsrapped version, this may be doing something else, maybe
            // adding them?!?
            cur_cmd.push(c);
        } else if token_type == DELIMINATOR && last_type == COMMAND {
            // end of command, need to execute now!
            print!("cur cmd: {}\n", cur_cmd);

            // TODO: make it evaluate the command based on definitions
            let mut is_name = true;
            let mut cur = String::from("");
            let mut defsidx = 0;
            let mut is_escaped = false;
            for (i, cur_char) in defs.chars().enumerate() {
                // if it's not a name definition, skip until it isn't
                // escape character will prevent evaluation
                if !is_escaped && cur_char == 'q' {
                    // q to terminate a name
                    defsidx = i;
                    cur = String::from("");
                    is_name = true;
                } else if !is_escaped && cur_char == '\\' {
                    // escape sequences
                    is_escaped = true;
                } else if !is_escaped && is_name && cur_char == 'n' {
                    // next char is an n, meaning end of the char sequence
                    // check if the name is correct, and then continue
                    // print!("cur: {}\n", cur);
                    if cur == cur_cmd {
                        break;
                    }
                    is_name = false;
                } else {
                    cur.push(cur_char);
                    print!("cur: \"{}\"\n", cur_char);
                }
            }

            //clear the command for the next chars
            cur_cmd = String::from("");
        } else if token_type != COMMAND && last_type == OPEN_PAREN {
            def = "npeq".to_string();
            //command that will by default evaluate all parameters
        }
        print!("found def! \"{}\"\n", def);
        last_type = token_type;

        i += 1;
        if i >= contents.len() {
            break;
        }
    }
} */

fn read_file(
    contents: &mut Vec<(char, u8, usize, usize)>,
    parens: &mut Vec<(usize, usize, usize, bool)>,
    commands: &mut Vec<(String, String)>,
    defs: &mut String,
) -> std::io::Result<()> {
    // contents: char, type, depth, delta
    // parens: start, end, command_location, matched
    // commands: command name, definition
    print!("reading file!\n");
    let mut tmp = String::new();
    let mut file = File::open(FILENAME)?;
    // in the bootstrapped code, this will be implemented by just reading all
    // characters to the array
    file.read_to_string(&mut tmp)?;
    //Defining token types
    //
    let mut depth = 0;
    let mut delta = 0;
    let mut token_type = OPEN_PAREN;
    let mut last_type = OPEN_PAREN;
    print!("code:\n{}\n", tmp);
    let mut full_command = String::from("");
    for (i, c) in tmp.chars().enumerate() {
        last_type = token_type;
        token_type = SOLO_PARAM;
        // update deltas and stuff for parens

        if last_type == SOLO_PARAM && token_type != SOLO_PARAM {
            //changing out of param to not param
            // add to delta
            // print!("exiting param\n");
            // WATCH OUT, ITER MIGHT BE WRONG, YOU MAY NEED TO DO
            // A len - i INSTEAD
            for (j_rev, (start, end, _, marker)) in parens.iter().rev().enumerate() {
                let j = parens.len() - j_rev - 1;
                // print!("backiter, j = {}\n", j);
                // print!("j: {}\tdepth: {}\n", j, depth);
                if *marker {
                    continue;
                }
                // adding 1 to account for zero indexing
                // - may remove at some point if it makes things cleaner
                delta = 1 + j - depth;
                break;
            }
            parens[depth + delta - 1].1 = i - 1;
            parens[depth + delta - 1].3 = true;
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
            // add to parens list
            parens.push((i, 0, 0, false));
            delta = parens.len() - depth;
            assert!(parens.len() == (depth + delta));
        } else if c == ')' {
            // move this to next iteration to keep close paren inside
            // the depth, next depth only applies on next iteration
            token_type = CLOSE_PAREN;

            // WATCH OUT, ITER MIGHT BE WRONG, YOU MAY NEED TO DO
            // A len - i INSTEAD
            //find delta with markers
            for (j_rev, (start, end, _, marker)) in parens.iter().rev().enumerate() {
                let j = parens.len() - j_rev - 1;
                if *marker {
                    continue;
                }
                // adding 1 to account for zero indexing
                // - may remove at some point if it makes things cleaner
                delta = 1 + j - depth;
                break;
            }

            // update parens param
            // print!("depth+delta-1 = {:?}\n", parens[depth + delta - 1]);
            parens[depth + delta - 1].1 = i;
            parens[depth + delta - 1].3 = true;
        } else if c == ' ' || c == '\t' || c == '\n' {
            // whitespace
            token_type = DELIMINATOR;
            if last_type == COMMAND {
                // ending a command, push command to command list if unique
                let def = find_def(defs.clone(), full_command.clone());
                print!(
                    "cmd: \"{}\"\tdef: {:?}\tcontains: {:?}\n",
                    full_command,
                    def,
                    commands.contains(&def)
                );
                if last_type == COMMAND {
                    if !commands.contains(&def) {
                        print!(
                            "found new command! {}\tputting at {}\n",
                            full_command,
                            commands.len()
                        );
                        // pushes the def as well
                        commands.push(def);
                        parens[depth + delta - 1].2 = commands.len() - 1;
                    } else {
                        // not a new def
                        parens[depth + delta - 1].2 =
                            commands.iter().position(|x| x == &def).unwrap();
                    }
                }
            }
        } else if last_type == OPEN_PAREN || last_type == COMMAND {
            // command
            // started with open paren
            token_type = COMMAND;
            // clear command if it's the beginning of a command
            if last_type == OPEN_PAREN {
                full_command = String::from("");
            }
            // push to command
            full_command.push(c);
            // print!(
            //     "pushing to full command! {}\tfull_command: {}\n",
            //     c, full_command
            // );
        } else if last_type == DELIMINATOR {
            // if last type was a deliminator
            // it must be a solo param
            // if it's going into a param, increase depth
            // print!("entering param\n");
            parens.push((i, i, 0, true));
            depth += 1;
            delta = parens.len() - depth;
            token_type = SOLO_PARAM;
        } else if last_type == SOLO_PARAM {
            // assuming that it is a mutli-char param
            // base case, defaulting to param
            // print!("base case, char: {}\n", c);
            depth += 1;
            // print!("depth: {}\tlen: {}\n", depth, parens.len());
            // print!("delta: {}\n", parens.len() - depth);

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
