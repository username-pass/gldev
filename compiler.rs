#![allow(warnings)]
use std::fs::File;
use std::io::prelude::*;

const FILENAME: &str = "test.gl";
const SOLO_PARAM: u8 = 0;
const COMMAND: u8 = 1;
const DELIMINATOR: u8 = 2;
const OPEN_PAREN: u8 = 3;
const CLOSE_PAREN: u8 = 4;
const DEBUG: bool = true;

#[derive(Debug, Clone)]
struct State {
    pub def_idx: usize,
    pub cur_idx: usize,
    pub cmd_loc: usize,
    pub start: usize,
    pub end: usize,
    pub dmode: u8,
    pub is_escaped: bool,
}

impl State {
    pub fn set_def_idx(&mut self, new_idx: usize) {
        self.def_idx = new_idx;
    }
    pub fn set_cur_idx(&mut self, new_idx: usize) {
        self.cur_idx = new_idx;
    }
    pub fn set_cmd_loc(&mut self, new_loc: usize) {
        self.cmd_loc = new_loc;
    }
    pub fn set_start(&mut self, new_start: usize) {
        self.start = new_start;
    }
    pub fn set_end(&mut self, new_end: usize) {
        self.end = new_end;
    }
    pub fn set_dmode(&mut self, new_dmode: u8) {
        self.dmode = new_dmode;
    }
    pub fn set_is_escaped(&mut self, is_escaped: bool) {
        self.is_escaped = is_escaped;
    }
    pub fn increment_def_idx(&mut self) {
        self.def_idx += 1;
    }
    pub fn increment_cur_idx(&mut self) {
        self.cur_idx += 1;
    }
    pub fn increment_cmd_loc(&mut self) {
        self.cmd_loc += 1;
    }
}

#[derive(Debug, Clone)]
struct StateHolder {
    states: Vec<State>,
}
impl StateHolder {
    pub fn push(&mut self, new_state: State) {
        self.states.push(new_state);
    }
    pub fn get_state(&mut self, idx: usize) -> &mut State {
        return &mut self.states[idx];
    }
    pub fn len(&self) -> usize {
        return self.states.len();
    }
}

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
    commands.push((String::from(""), String::from("pw"), String::from("q")));
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
    let bytecode = do_bytecode(&mut contents, &mut parens, commands, defs);
    print!("Got bytecode!\n===\n{}\n===\n", bytecode);
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
    // temp test thing to print a string
    // I don't know why this needs the extra escape characters,
    // even though the rest are fine. Maybe look into it??
    // defs.push_str("tmpwritenxnsthis\\\\ is\\ a\\ test\\ output q");
    // temp thing to test pushing and writing strings
    defs.push_str("bcnxnpwq");
    print!("defs: {}", defs);
}

fn do_bytecode(
    contents: &mut Vec<(char, u8, usize, usize)>,
    // contents: (char, type, depth, delta)
    parens: &mut Vec<(usize, usize, usize, bool)>,
    // parens: (start, end, command_loc, marker)
    commands: Vec<(String, String, String)>,
    // commands: (name, def)
    defs: String,
) -> String {
    let SEARCHING_CMD = 0;
    let WRITING = 1;
    let SEARCHING_PARAM = 2;
    let EVALUATING = 3;
    let STRING_WRITING = 4;
    // this is the state at a given param
    // (definition_idx, cur_idx, cmd_loc, start, end, mode, is_escaped)
    // let mut state: Vec<(usize, usize, usize, usize, usize, u8, bool)> = Vec::new();
    let mut states: StateHolder = StateHolder { states: Vec::new() };
    let mut to_push: Option<State> = None;
    // state holds the state like this:
    // each depth has its own value in the following format:
    // (definition_idx, cur_idx, cmd_loc, start, end, mode, is_escaped)
    let mut global_idx = 0;
    let mut last_type = CLOSE_PAREN;
    let mut is_escaped = false;
    let mut command_stack: Vec<(usize, usize)> = Vec::new();
    let mut mode = SEARCHING_CMD;
    let mut cur_state_idx = 0;
    let mut output = String::from("");
    let mut new_mode = SEARCHING_CMD;
    // this should work as each item in the loop does one thing. There shouldn't be
    // too many nested layers.
    // The exception is writing a string, where it just repeatedly writes the string
    // that might also change
    // the idea is that you change modes, and each mode does one different thing
    // so write might write the current char, and search cmd might just move to the
    // next cmd
    // find location of command
    states.push(State {
        def_idx: 0,           // index within definition of the current command
        cur_idx: 0,           // index within the actual code being iterated on
        cmd_loc: parens[0].2, // location of command in command array
        start: parens[0].0,   // start index of the current parameter
        end: parens[0].1,     // index of final char of current param
        dmode: SEARCHING_CMD, // the current mode of the full state
        is_escaped: false,    // whether or not the current char is escaped
    });
    loop {
        match to_push {
            Some(ref state) => {
                states.push(state.clone());
                to_push = None;
                continue;
            }
            None => {}
        }
        let states_len = states.len();
        // global_idx is to find the current mode and all that
        if global_idx >= contents.len() {
            break;
        }
        let mut cur_state = states.get_state(cur_state_idx);
        // if you reach the end of the param, re-evaluate with previous param
        if cur_state.cur_idx > cur_state.end {
            // finished the param, go back to prev
            let (c_new, token_type_new, depth_new, delta_new) = contents[cur_state.cur_idx];
            let (start_new, end_new, cmd_loc_new, _) = parens[depth_new + delta_new - 1];
            cur_state_idx = depth_new + delta_new - 1;

            print!(
                "new state!, max_len: {}\tcur_state_idx: {}\n",
                contents.len(),
                cur_state_idx,
            );
            // if it's the base case, break
            if cur_state_idx == 0 {
                break;
            }
            // found new state, now start re-evaluating
            continue;
        }

        // now to evaluate the modes
        if cur_state.dmode == SEARCHING_CMD {
            // looking for next command, so increment idx until it's a command
            let (cmd, default, def) = &commands[cur_state.cmd_loc];
            let next_cmd = def.chars().nth(cur_state.def_idx).unwrap();
            let (c, token_type, depth, delta) = contents[cur_state.cur_idx];
            if token_type == COMMAND {
                // change the state to the new command
                cur_state_idx = depth + delta - 1;
                if cur_state_idx >= states_len {
                    print!(
                        "cur state is {}, state len is {}",
                        cur_state_idx, states_len
                    );

                    to_push = Some(State {
                        def_idx: 0,
                        cur_idx: cur_state.cur_idx,
                        cmd_loc: cur_state.cmd_loc,
                        start: cur_state.start,
                        end: cur_state.end,
                        dmode: SEARCHING_CMD,
                        is_escaped: false,
                    });
                }
                //this is a command, break and continue to evaluation
                cur_state.dmode = EVALUATING;
                continue;
            }
            // no command found
            // continue searching
            cur_state.increment_cur_idx();
            continue;
        } else if cur_state.dmode == EVALUATING {
            //this is the switchboard to find what to do next based on the command
            let (c, token_type, depth, delta) = contents[cur_state.cur_idx];
            let (cmd, default, def) = &commands[cur_state.cmd_loc];
            let next_cmd = def.chars().nth(cur_state.def_idx).unwrap();
            print!(
                "c: {}\tcmd: {}\tidx: {}\tlen: {}\tdefault: \"{}\"\tdef: \"{}\"\n",
                c,
                cmd,
                cur_state.def_idx,
                def.len(),
                default,
                def,
            );

            if next_cmd == 'x' {
                // no-op, do nothing
                cur_state.increment_def_idx();
                continue;
            } else if next_cmd == 's' {
                print!("String command!\n");
                // command is to write a string
                cur_state.dmode = STRING_WRITING;
                cur_state.increment_def_idx();
                continue;
                default = &String::from("");
            } else if next_cmd == 'p' {
                // push value to stack
                cur_state.dmode = SEARCHING_PARAM;
                continue;
            } else if next_cmd == 'e' {
                cur_state.dmode = SEARCHING_PARAM;
                continue;
            } else if next_cmd == 'w' {
                cur_state.dmode = SEARCHING_PARAM;
                continue;
            } else if next_cmd == 'q' {
                // end of command, going to next command
                cur_state.dmode = SEARCHING_CMD;
                cur_state.cur_idx = cur_state.end;
            } else {
                // this probably shouldn't be hit, so just panic
                assert!(next_cmd == '2');
            }
        } else if cur_state.dmode == STRING_WRITING {
            let (cmd, default, def) = &commands[cur_state.cmd_loc];
            let cur_char = def.chars().nth(cur_state.def_idx).unwrap();
            print!("writing string. cur_char: {}\n", cur_char);
            if !(cur_state.is_escaped) && cur_char == '\\' {
                cur_state.is_escaped = true;
                cur_state.increment_def_idx();
                continue;
            } else if !(cur_state.is_escaped) && cur_char == ' ' {
                // end of string to write
                cur_state.dmode = EVALUATING;
                cur_state.increment_def_idx();
                continue;
            } else if !(cur_state.is_escaped) && cur_char == 'q' {
                // end of definition, refer to EVALUATING mode to deal with that
                cur_state.dmode = EVALUATING;
                continue;
            } else if cur_state.is_escaped && cur_char == 'n' {
                // \n character, return newline
                output.push('\n');
                cur_state.increment_def_idx();
                continue;
            } else if cur_state.is_escaped && cur_char == 't' {
                // \t character, return tab
                output.push('\t');
                cur_state.increment_def_idx();
                continue;
            } else {
                // reset being escaped
                cur_state.is_escaped = false;
                output.push(cur_char);
                cur_state.increment_def_idx();
            }
        } else if cur_state.dmode == SEARCHING_PARAM {
            print!("Searching param\n");
            let (cmd, default, def) = &commands[cur_state.cmd_loc];
            let next_cmd = def.chars().nth(cur_state.def_idx).unwrap();
            let (c, token_type, depth, delta) = contents[cur_state.cur_idx];
            let (param_start, param_end, _, _) = parens[depth + delta - 1];
            if token_type != OPEN_PAREN && token_type != SOLO_PARAM {
                // haven't found param, keep looking
                cur_state.increment_cur_idx();
                continue;
            }

            // have found param, execute the next token
            let cur_char = def.chars().nth(cur_state.def_idx).unwrap();
            print!("found param: {:?}\n", contents[cur_state.cur_idx]);
            if cur_char == 'p' {
                command_stack.push((param_start, param_end));
                cur_state.increment_def_idx();
                cur_state.dmode = EVALUATING;
                print!("next cmd: {}\n", next_cmd);
                continue;
            } else if cur_char == 'e' {
                // mode = EVALUATING;
                cur_state_idx = depth + delta - 1;
                if cur_state_idx >= states_len {
                    print!("cur_state_idx {}\tstate len: {}", cur_state_idx, states_len);

                    to_push = Some(State {
                        def_idx: 0,
                        cur_idx: cur_state.cur_idx,
                        cmd_loc: cur_state.cmd_loc,
                        start: param_start,
                        end: param_end,
                        dmode: WRITING,
                        is_escaped: false,
                    });

                    let (c, token_type, depth, delta) = contents[cur_state.cur_idx];
                }
                new_mode = WRITING; // set the state to be WRITING
                cur_state.dmode = SEARCHING_CMD;
            } else if cur_char == 'w' {
                // move to next state to write
                cur_state_idx = depth + delta - 1;
                if cur_state_idx >= states_len {
                    print!("cur_state_idx {}\tstate len: {}", cur_state_idx, states_len);

                    to_push = Some(State {
                        def_idx: 0,
                        cur_idx: cur_state.cur_idx,
                        cmd_loc: cur_state.cmd_loc,
                        start: param_start,
                        end: param_end,
                        dmode: WRITING,
                        is_escaped: false,
                    });
                } else {
                    cur_state = states.get_state(cur_state_idx);
                }
                cur_state.set_dmode(WRITING);
                // state[cur_state_idx].5 = WRITING; // set the state to be WRITING
                // cur_state.set_dmode(WRITING) // make it write the parameter verbatim
                print!(
                    "======\n\nidx: {}\tchar: {}\tdmode: {}\n",
                    cur_state.cur_idx, c, cur_state.dmode
                );
                continue;
            }
        } else if cur_state.dmode == WRITING {
            // ref mutcur_state.def_idx,
            // ref mut cur_idx,
            // cur_state.cmd_loc,
            // start,
            // end,
            // ref mut dmode,
            // ref mutcur_state.is_escaped,
            print!(
                "c, type, depth, delta:\t{:?}\n",
                contents[cur_state.cur_idx]
            );
            if cur_state.cur_idx >= cur_state.end {
                print!(
                    "ending, ({}\t{})\tidx: {}",
                    cur_state.start, cur_state.end, cur_state.cur_idx
                );
                cur_state.dmode = EVALUATING;
                cur_state.increment_def_idx();
                // depth and delta of next character (should be previous state)
                let (_, _, depth_next, delta_next) = contents[cur_state.cur_idx + 1];
                continue;
            }
            /* if cur_state.cur_idx == start || cur_state.cur_idx == end {
                print!("start, end: ({}\t{}) idx: {}\n", cur_state.start, cur_state.end, cur_state.cur_idx);
                // at beginning or end, time to test
                let (param_start, param_end, _, _) = parens[depth + delta - 1];
                print!(
                    "at edge, char: {}\tidx: {}\ttoken_type: {}\n",
                    c, cur_state.cur_idx, token_type
                );
                if token_type == CLOSE_PAREN {
                    print!("cur_state.ending...\n");
                    cur_state.dmode = EVALUATING;
                    cur_state.def_idx += 1;
                    cur_state.cur_idx = param_start;
                    continue;
                }
            } */
            // let (c, token_type, depth, delta) = contents[cur_state.cur_idx];
            output.push(contents[cur_state.cur_idx].0);
            cur_state.increment_cur_idx();
        } else if false && cur_state.dmode == WRITING {
            // this is for writing the parameter verbatim
            let (cmd, default, def) = &commands[cur_state.cmd_loc];
            let (c, token_type, depth, delta) = contents[cur_state.cur_idx];
            let (start_char, end_char, _, _) = parens[depth + delta - 1];
            if cur_state.cur_idx >= cur_state.end {
                // end of param, move back and repeat
                // if it's not a paren, then print it as well
                if token_type != CLOSE_PAREN {
                    output.push(c);
                }
                print!("output: \n{}\n", output);
                print!(
                    "end of param, cur_state.end: {}\tc: {}\ttoken_type: {}\n",
                    cur_state.end, c, token_type
                );
                // get previous character (entry of param), and
                // go to beginning of param
                let (c_prev, token_type_prev, depth_prev, delta_prev) =
                    contents[cur_state.cur_idx - 1];
                let (start_prev, end_prev, _, _) = parens[depth_prev + delta_prev - 1];
                print!("start, cur_state.end: ({}\t{})\n", start_prev, end_prev);
                cur_state.cur_idx = start_prev;
                cur_state.dmode = EVALUATING;
                continue;
                // assert!(c == '1');
            } else if cur_state.cur_idx == cur_state.start && c == '(' {
                // if it's an open paren, continue so that it doesn't worry
                // about the beginnings of parenthesized params
                cur_state.increment_cur_idx();
                continue;
                print!(" at start! char = {}\n", c);
            }
            output.push(c);
            cur_state.increment_cur_idx();
            // break for now
            // assert!(c == '1');
        }
    }
    return output;
    /* loop {
        if true || global_idx >= contents.len() {
            break;
        }
        print!("idx: {}, mode: {}, state: {:?}\n", global_idx, mode, state);

        if mode == SEARCHING_CMD {
            // look for next command to execute
            let (c, token_type, depth, delta) = contents[global_idx];
            let (start, end, cmd_loc, _) = parens[depth + delta - 1];
            cur_depth = depth;
            if token_type == COMMAND {
                loop {
                    if state.len() >= depth + delta - 1 {
                        print!(
                            "breaking, len: {}\td+d-1: {}\n",
                            state.len(),
                            depth + delta - 1
                        );
                        break;
                    }
                    // junk items, should never be used, in theory
                    state.push((0, 0, 0, 0));
                }
                state.push((0, global_idx, cmd_loc, delta));
                print!("len: {}\tidx: {}\n", state.len(), global_idx);
                // found command, need to start evaluating
                mode = SEARCHING_PARAM;
            } else {
                // keep searching
                global_idx += 1;
            }
        } else if mode == SEARCHING_PARAM {
            let (c, token_type, depth, delta) = contents[global_idx];
            if token_type == OPEN_PAREN || token_type == SOLO_PARAM {
                mode = EVALUATING;
            } else {
                global_idx += 1;
            }
        } else if mode == EVALUATING {
            // found param, need to start evaluating previous index
            let (c_prev, token_type_prev, depth_prev, delta_prev) = contents[global_idx - 1];
            let (start, end, cmd_loc, _) = parens[depth_prev + delta_prev - 1];
            let (cmd, def) = &commands[cmd_loc];
            // get cmd def, and try to find what to do next

            let (def_idx, _, _, _) = &mut state[depth_prev + delta_prev - 1];
            let cur_def_cmd = def.chars().nth(*def_idx).unwrap();
            print!("cur_def_cmd: {}\n", cur_def_cmd);
            if cur_def_cmd == 'x' {
                // no-op, do nothing
            } else if cur_def_cmd == 's' {
                //write value to string
                mode = STRING_WRITING;
            } else if cur_def_cmd == 'p' {
                // push value to stack
                let (c, token_type, depth, delta) = contents[global_idx];
                let (param_start, param_end, _, _) = parens[depth + delta - 1];
                print!("pushing to stack, ({},\t{})\n", param_start, param_end);
                command_stack.push((param_start, param_end));
            } else if cur_def_cmd == 'e' {
                // start evaluating again
            } else if cur_def_cmd == 'w' {
                // start writing the current param to output
            }
        } else if mode == WRITING {
            let (c_prev, token_type_prev, depth_prev, delta_prev) = contents[global_idx - 1];
            let (start, end, cmd_loc, _) = parens[depth_prev + delta_prev - 1];
            let (cmd, def) = &commands[cmd_loc];
            // get cmd def, and try to find what to do next
            let (def_idx, _, _, _) = &mut state[depth_prev + delta_prev - 1];
            let cur_char = def.chars().nth(*def_idx).unwrap();
            if cur_char == 'n' && !is_escaped {
                // end of the param
                mode = SEARCHING_PARAM;
                *def_idx += 1;
            } else if cur_char == 'q' && !is_escaped {
                // end of def, look for next command
                mode = SEARCHING_CMD
            } else if cur_char == ' ' && !is_escaped {
                // end of string to write
                // TODO: change this to something more descriptive
                // maybe something like "finding next mode"
                mode = EVALUATING;
                *def_idx += 1;
            } else if cur_char == '\\' && !is_escaped {
                is_escaped = true;
                *def_idx += 1;
            } else {
                is_escaped = false;
                output.push(cur_char);
            }
        } else if mode == EVALUATING {
            // evaluating the parameters
            let (c, token_type, depth, delta) = contents[global_idx];
            let &mut (start, end, cmd_loc, _) = &mut parens[depth + delta - 1];
            let (cmd, def) = &commands[cmd_loc];
            let (definition_idx, cur_idx, cmd_loc, delta) = &mut state[depth + delta - 1];
            let cur_def_cmd = def.chars().nth(*definition_idx).unwrap();
            if cur_def_cmd == 'p' {
                //push value to stack
                command_stack.push((start, end))
            } else if cur_def_cmd == 'w' {
                loop {}
            }

            *definition_idx += 1;
        }
    } */
}

fn find_def(defs: String, searchfor: String) -> (String, String, String) {
    // now find definition
    let mut def_idx: usize = 0; // the position in the definition vec
    let mut def_start = 0; // the start of the current definition
    let mut cmd = String::from(""); // the actual command
    let mut default = String::from(""); // the default parameter execution
    let mut def = String::from("pwq"); // the full definition
    let mut is_escaped = false;
    let mut looking_for = 0; // 0 = command, 1 = default, 2 = everything else
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
                return (cmd, default, def);
                // break;
            }

            // end of def, reset
            // set start to next char
            def_start = def_idx + 1;
            // make it look for the command
            looking_for = 0;
            // set def to blank
            def = String::from("");
            default = String::from("");
            cmd = String::from("");
        }
        // next param
        else if cur_char == 'n' && !is_escaped {
            if looking_for == 0 {
                looking_for = 1;
            } else if looking_for == 1 {
                looking_for = 2;
            }
        } else if looking_for == 0 {
            //     // print!("added to def: {}\n", cur_char);
            // if we're looking for a command, add to command
            cmd.push(cur_char);
        } else if looking_for == 1 {
            // now looking for default
            default.push(cur_char);
        } else {
            //     // print!("added to cmd {}\n", cur_char);
            // otherwise, just add to general definition
            def.push(cur_char);
        }

        // add char when not command

        // // print!("{}", cur_char);

        def_idx += 1;
    }
    return (cmd, default, def);
}

fn read_file(
    contents: &mut Vec<(char, u8, usize, usize)>,
    parens: &mut Vec<(usize, usize, usize, bool)>,
    commands: &mut Vec<(String, String, String)>,
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
