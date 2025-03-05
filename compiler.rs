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
    pub def_idx: usize,      // location within the definition
    pub cur_idx: usize,      // location within the content
    pub cmd_loc: usize,      // index of the command definition
    pub start: usize,        // index of the start of the parameter
    pub end: usize,          // index of the end of the parameter
    pub dmode: u8,           // the mode that it is currently in
    pub is_escaped: bool,    // whether or not something is escaped
    pub callback_idx: usize, // the index of the state to callback to if required
    pub def_item: Command,   // TODO: add "next char" and other helper functions
}

impl State {
    pub fn default() -> State {
        State {
            def_idx: 0,                 // index within definition of the current command
            cur_idx: 0,                 // index within the actual code being iterated on
            cmd_loc: parens[0].cmd_loc, // location of command in command array
            start: parens[0].start,     // start index of the current parameter
            end: parens[0].end,         // index of final char of current param
            dmode: SEARCHING_CMD,       // the current mode of the full state
            is_escaped: false,          // whether or not the current char is escaped
            callback_idx: 0,            // the index of the callback state when needed
            def_item: Command {},
        }
    }
    pub fn set_def_idx(&mut self, new_idx: usize) {
        self.def_idx = new_idx;
        return self;
    }
    pub fn set_cur_idx(&mut self, new_idx: usize) {
        self.cur_idx = new_idx;
        return self;
    }
    pub fn set_cmd_loc(&mut self, new_loc: usize) {
        self.cmd_loc = new_loc;
        return self;
    }
    pub fn set_start(&mut self, new_start: usize) {
        self.start = new_start;
        return self;
    }
    pub fn set_end(&mut self, new_end: usize) {
        self.end = new_end;
        return self;
    }
    pub fn set_dmode(&mut self, new_dmode: u8) {
        self.dmode = new_dmode;
        return self;
    }
    pub fn set_is_escaped(&mut self, is_escaped: bool) {
        self.is_escaped = is_escaped;
        return self;
    }
    pub fn set_def_item(&mut self, new_def_item: Command) {
        self.def_item = new_def_item;
        return self;
    }
    pub fn set_callback_idx(&mut self, new_callback_idx: usize) {
        self.callback_idx = new_callback_idx;
        return self;
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
    pub fn jump_to_end(&mut self) {
        self.cur_idx = self.end;
    }
    pub fn jump_to_start(&mut self) {
        self.cur_idx = self.start;
    }
    // TODO: make it smarter and store internally(?)
    pub fn next_def_cmd(&mut self) -> char {
        return self.def_item.def.chars().nth(self.def_idx);
    }
}

struct Code_character {
    // (char, type, depth, delta)
    pub cur_char: char,
    pub token_type: u8,
    pub depth: usize,
    pub delta: usize,
}

impl Code_character {
    pub fn depth_plus_delta(self) {
        return self.depth + self.delta;
    }
}

struct Paren {
    pub start: usize,
    pub end: usize,
    pub cmd_loc: usize,
    pub marker: bool,
}

struct Command {
    pub name: String,
    pub default: String,
    pub def: String,
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
    commands.push(Command {
        name: String::from(""),
        default_def: String::from("pw"),
        def: String::from("peq"),
    });
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
        for (i, (cmd, default, def)) in commands.iter().enumerate() {
            print!("{}\t|\t{}\t{}\t{}\n", i, cmd, default, def);
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
    defs.push_str("Nnxnsinit\\\\  pws\n q");
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
    defs.push_str("bcnxnpwnpwq");
    defs.push_str("testnxnsa pwnpwq");
    print!("defs: {}", defs);
}

fn do_bytecode(
    contents: &mut Vec<Code_character>,
    parens: &mut Vec<Paren>,
    commands: &mut Vec<Command>,
    defs: String,
) -> String {
    let SEARCHING_CMD = 0;
    let EVALUATING = 1;
    let SEARCHING_PARAM = 2;
    let WRITING = 3;
    let STRING_WRITING = 4;

    let mut output = String::from("");
    let mut states: StateHolder = StateHolder { states: Vec::new() };
    let mut command_stack: Vec<Paren> = Vec::new();
    let mut cur_state_idx = 0;
    let mut to_push: Option<State> = None;

    states.push(State {
        def_idx: 0,                 // index within definition of the current command
        cur_idx: 0,                 // index within the actual code being iterated on
        cmd_loc: parens[0].cmd_loc, // location of command in command array
        start: parens[0].start,     // start index of the current parameter
        end: parens[0].end,         // index of final char of current param
        dmode: SEARCHING_CMD,       // the current mode of the full state
        is_escaped: false,          // whether or not the current char is escaped
        callback_idx: 0,            // the index of the callback state when needed
    });

    // this is a rewrite based on a tree diagram I wrote
    loop {
        let cur_state = states.get_state(cur_state_idx);
        if cur_state.cur_idx >= contents.len() {
            break;
        }
        if cur_state.dmode == SEARCHING_CMD {
            cur_state.increment_cur_idx();
            if contents[cur_state.cur_idx].token_type == COMMAND {
                cur_state.dmode = EVALUATING;
            }
            // go back to next state
            continue;
        } else if cur_state.dmode == EVALUATING {
            cur_state.increment_def_idx();
            match cur_state.next_def_cmd() {
                x if x == 'x' => println!("Input is equal to a"),
                x if x == 'q' => notimplemented!(),
                x if x == 'n' => {
                    cur_state.jump_to_end();
                    cur_state.increment_cur_idx();
                    cur_state.dmode = SEARCHING_PARAM;
                    continue;
                }
                x if x == 'p' => {
                    if contents[cur_state.cur_idx].depth < contents[cur_state.start].depth {
                        // if it's inside a param or not
                        command_stack.push(parens[contents[cur_state.cur_idx].depth_plus_delta()])
                    }
                }
            }

            if cur_state.next_def_cmd() == 'x' {
                continue;
            }
        }
    }
}

// pre board-rewrite
fn do_bytecode_way_less_old(
    contents: &mut Vec<(char, u8, usize, usize)>,
    // contents: (char, type, depth, delta)
    parens: &mut Vec<(usize, usize, usize, bool)>,
    // parens: (start, end, command_loc, marker)
    commands: Vec<(String, String, String)>,
    // commands: (name, def)
    defs: String,
) -> String {
    let SEARCHING_CMD = 0;
    let EVALUATING = 1;
    let SEARCHING_PARAM = 2;
    let WRITING = 3;
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
        callback_idx: 0,      // the index of the callback state when needed
    });
    let mut iterations = 0; //DEBUG, remove
    loop {
        iterations += 1;
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
        if cur_state.cur_idx >= contents.len() {
            // let (c, token_type, depth, delta) = contents[cur_state.cur_idx - 1];
            // // print!("{:?}\n", contents[cur_state.cur_idx - 1]);

            break;
        }

        // let next_cmd = def.chars().nth(cur_state.def_idx).unwrap();
        print!(
            "===\niter: {}\tcur_state: {} cur_char: {} next_cmd: {} def: \"{}\"\n{:?}\n{:?}\n",
            iterations,
            cur_state_idx,
            contents[cur_state.cur_idx].0,
            commands[cur_state.cmd_loc]
                .2
                .chars()
                .nth(cur_state.def_idx)
                .unwrap(),
            commands[cur_state.cmd_loc].2,
            cur_state,
            command_stack
        );
        if commands[cur_state.cmd_loc]
            .2
            .chars()
            .nth(cur_state.def_idx)
            .unwrap()
            == 'n'
            && cur_state.dmode != STRING_WRITING
        {
            assert!("we reached" == "an 'n'");
        }

        //     assert!("curstate" == "n");
        // }

        // now to evaluate the modes
        if cur_state.dmode == SEARCHING_CMD {
            // looking for next command, so increment idx until it's a command
            let (cmd, default, def) = &commands[cur_state.cmd_loc];
            let (c, token_type, depth, delta) = contents[cur_state.cur_idx];
            let (param_start, param_end, param_cmd_loc, _) = parens[depth + delta - 1];
            let next_cmd = def.chars().nth(cur_state.def_idx).unwrap();

            if depth + delta - 1 >= states_len {
                // print!(
                //     "writing, starting at: {:?}\nparam start, end ({}\t{})\n",
                //     contents[cur_state.cur_idx], param_start, param_end
                // );
                //     print!(
                //         "cur state is {}, state len is {}, mode is {}\n",
                //         depth + delta - 1,
                //         states_len,
                //         cur_state.dmode
                //     );

                to_push = Some(State {
                    def_idx: 0,
                    cur_idx: cur_state.cur_idx,
                    cmd_loc: param_cmd_loc,
                    start: param_start,
                    end: param_end,
                    dmode: SEARCHING_CMD,
                    is_escaped: false,
                    callback_idx: cur_state_idx,
                });
                continue;
            }
            if token_type == COMMAND {
                // print!(
                //     "writing, starting at: {:?}\nparam start, end ({}\t{})\n",
                //     contents[cur_state.cur_idx], param_start, param_end
                // );
                // change the state to the new command
                cur_state_idx = depth + delta - 1;
                cur_state = states.get_state(cur_state_idx);

                //     print!("idx: {}, len: {}\n", cur_state_idx, states_len);
                assert!(cur_state_idx <= states_len);
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
            if def.is_empty() {
                //     print!("no def! time to continue onward...\n");
                cur_state.dmode = SEARCHING_CMD;
                cur_state.cur_idx = cur_state.end + 1;
                continue;
            }
            // print!(
            //     "cmd: {}\tdef: {}\tdef_idx: {}\tcmd_loc: {}\n",
            //     cmd, def, cur_state.def_idx, cur_state.cmd_loc
            // );
            let next_cmd = def.chars().nth(cur_state.def_idx).unwrap();
            // print!(
            //     "c: {}\tcmd: {}\tidx: {}\tcur_idx: {}\tdefault: \"{}\"\tdef: \"{}\"\tnext: {}\n",
            //     c, cmd, cur_state.def_idx, cur_state.cur_idx, default, def, next_cmd
            // );
            print!(
                "next cmd: {}, idx: {}\t{:?}\n",
                next_cmd, cur_state.def_idx, cur_state
            );

            if next_cmd == 'x' {
                // no-op, do nothing
                cur_state.increment_def_idx();
                continue;
            } else if next_cmd == 's' {
                //     print!("String command!\n");
                // command is to write a string
                cur_state.dmode = STRING_WRITING;
                cur_state.increment_def_idx();
                continue;
                default = &String::from("");
            } else if next_cmd == 'p' {
                // push value to stack
                print!("setting to searching param");
                cur_state.dmode = SEARCHING_PARAM;
                continue;
            } else if next_cmd == 'e' {
                // pop top command and execute
                let (cmd_start, cmd_end) = command_stack.pop().unwrap();
                cur_state.cur_idx = cmd_start;
                cur_state.dmode = EVALUATING;
                // cur_state.increment_cur_idx();
                continue;
            } else if next_cmd == 'w' {
                let (cmd_start, cmd_end) = command_stack.pop().unwrap();
                // cur_state.cur_idx = cmd_start;
                let (new_c, new_type, new_depth, new_delta) = contents[cmd_start];

                cur_state_idx = new_depth + new_delta - 1;
                assert!(cur_state_idx < states_len);

                cur_state = states.get_state(cur_state_idx);
                cur_state.set_dmode(WRITING);

                // cur_state.dmode = SEARCHING_PARAM;
                cur_state.increment_cur_idx();
                continue;
            } else if next_cmd == 'q' {
                // end of command, going to next command
                cur_state.dmode = SEARCHING_CMD;
                cur_state.cur_idx = cur_state.end + 1;
            } else if next_cmd == 'n' {
                print!("Next param\n");
                cur_state.increment_def_idx();
                cur_state.dmode = SEARCHING_PARAM;
                continue;
            } else {
                // this probably shouldn't be hit, so just panic
                //     print!(
                //         "next_cmd \"{}\", idx: {}, def {}\n",
                //         next_cmd, cur_state.def_idx, def
                //     );
                assert!(next_cmd == '2');
            }
        } else if cur_state.dmode == STRING_WRITING {
            let (cmd, default, def) = &commands[cur_state.cmd_loc];
            let cur_char = def.chars().nth(cur_state.def_idx).unwrap();
            // print!("writing string. cur_char: {}\n", cur_char);
            if !(cur_state.is_escaped) && cur_char == '\\' {
                cur_state.is_escaped = true;
                cur_state.increment_def_idx();
                continue;
            } else if !(cur_state.is_escaped) && cur_char == ' ' {
                // end of string to write
                //     print!(
                //         "ending space, idx: {}\tis_escaped: {}\n",
                //         cur_state.def_idx, cur_state.is_escaped
                //     );
                cur_state.dmode = EVALUATING;
                cur_state.increment_def_idx();
                continue;
            } else if !(cur_state.is_escaped) && cur_char == 'q' {
                // end of definition, refer to EVALUATING mode to deal with that
                cur_state.dmode = EVALUATING;
                continue;
            } else if cur_state.is_escaped && cur_char == 'n' {
                // TODO: FIX FOR INIT!!!!
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
                continue;
            }
        } else if cur_state.dmode == SEARCHING_PARAM {
            let (cmd, default, def) = &commands[cur_state.cmd_loc];
            let next_cmd = def.chars().nth(cur_state.def_idx).unwrap();
            let (c, token_type, depth, delta) = contents[cur_state.cur_idx];
            let (param_start, param_end, param_cmd_loc, _) = parens[depth + delta - 1];
            // print!("Searching param {}\n", c);
            if token_type != OPEN_PAREN && token_type != SOLO_PARAM {
                // haven't found param, keep looking
                cur_state.increment_cur_idx();
                continue;
            }

            // have found param, execute the next token
            let cur_char = def.chars().nth(cur_state.def_idx).unwrap();
            // print!("found param: {:?}\n", contents[cur_state.cur_idx]);
            if cur_char == 'p' {
                print!("====\n\nPUSHING START AND END\n\n====\n");
                command_stack.push((param_start, param_end));
                cur_state.increment_def_idx();
                cur_state.dmode = EVALUATING;
                // print!(
                //     "pushed start end ({}\t{}) {}\n",
                //     param_start,
                //     param_end,
                //     depth + delta - 1
                // );
                //     print!("next cmd: {}\n", next_cmd);
                cur_state.set_cur_idx(cur_state.start);
                continue;
            } else {
                print!("cur_char: {}\n", cur_char);
                // assert!(cur_char == '3');
                cur_state.set_dmode(EVALUATING);
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
            // print!(
            //     "c, type, depth, delta:\t{:?}\n",
            //     contents[cur_state.cur_idx]
            // );
            if cur_state.cur_idx <= cur_state.start {
                //     print!(
                //         "starting!, {}\t{}\n",
                //         cur_state.cur_idx, contents[cur_state.cur_idx].0
                //     );
                if contents[cur_state.cur_idx].1 == OPEN_PAREN {
                    // if open paren at beginning, skip
                    cur_state.increment_cur_idx();
                    continue;
                }
            }
            if cur_state.cur_idx >= cur_state.end {
                print!(
                    "ending, def_idx: {}\tend: {}\tidx: {}\n",
                    cur_state.def_idx, cur_state.end, cur_state.cur_idx
                );
                cur_state.dmode = SEARCHING_CMD;
                // cur_state.increment_def_idx();
                // depth and delta of next character (should be previous state)
                let (_, _, depth_next, delta_next) = contents[cur_state.cur_idx + 1];
                if contents[cur_state.cur_idx].1 == SOLO_PARAM {
                    // if it's a solo param, push the last char
                    output.push(contents[cur_state.cur_idx].0);
                }
                // go back to beginning
                cur_state.set_cur_idx(cur_state.start);
                cur_state_idx = cur_state.callback_idx; // go back to original state
                cur_state = states.get_state(cur_state_idx);
                cur_state.increment_def_idx();
                print!("calling back to {}\n", cur_state_idx);
                continue;
            }
            output.push(contents[cur_state.cur_idx].0);
            cur_state.increment_cur_idx();
            continue;
        }
        print!("dmode: {}\n", cur_state.dmode);
        assert!("a" == "b");
    }
    return output;
}

fn find_def(defs: String, searchfor: String) -> (String, String, String) {
    // now find definition
    let mut def_idx: usize = 0; // the position in the definition vec
    let mut def_start = 0; // the start of the current definition
    let mut cmd = String::from(""); // the actual command
    let mut default = String::from("x"); // the default parameter execution
    let mut def = String::from("xq"); // the full definition
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

            print!("reached end, checking...\n");
            print!(
                "is equal: {}\tsearch_for: {}\tcmd: {}\tdef: {}\n",
                searchfor == cmd,
                searchfor,
                cmd,
                def
            );
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
            } else {
                def.push(cur_char);
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
        if is_escaped {
            is_escaped = false;
        }
    }
    print!(
        "found def: {:?}\n",
        (cmd.clone(), default.clone(), def.clone())
    );
    if default.clone().is_empty() {
        default = String::from("x");
    }
    if def.clone().is_empty() {
        def = String::from("xq");
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
                // print!(
                //     "cmd: \"{}\"\tdef: {:?}\tcontains: {:?}\n",
                //     full_command,
                //     def,
                //     commands.contains(&def)
                // );
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
