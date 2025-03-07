#!allow(warnings)]
use std::fs::File;
use std::io::prelude::*;

const FILENAME: &str = "test.gl";

const SOLO_PARAM: u8 = 0;
const COMMAND: u8 = 1;
const DELIMINATOR: u8 = 2;
const OPEN_PAREN: u8 = 3;
const CLOSE_PAREN: u8 = 4;

const SEARCHING_CMD: u8 = 0;
const EVALUATING: u8 = 1;
const SEARCHING_PARAM: u8 = 2;
const WRITING: u8 = 3;
const STRING_WRITING: u8 = 4;

const DEBUG: bool = true;

#[derive(Debug, Clone, Copy)]
struct State {
    pub def_idx: usize,   // location within the definition
    pub cur_idx: usize,   // location within the content
    pub cmd_loc: usize,   // index of the command definition
    pub start: usize,     // index of the start of the parameter
    pub end: usize,       // index of the end of the parameter
    pub dmode: u8,        // the mode that it is currently in
    pub is_escaped: bool, // whether or not something is escaped
    pub callback_idx: usize, // the index of the state to callback to if required
                          // pub def_item: Command,   // TODO: add "next char" and other helper functions
}

impl State {
    pub fn new() -> State {
        return State::default();
    }
    pub fn default() -> State {
        State {
            def_idx: 0,           // index within definition of the current command
            cur_idx: 0,           // index within the actual code being iterated on
            cmd_loc: 0,           // location of command in command array
            start: 0,             // start index of the current parameter
            end: 0,               // index of final char of current param
            dmode: SEARCHING_CMD, // the current mode of the full state
            is_escaped: false,    // whether or not the current char is escaped
            callback_idx: 0,      // the index of the callback state when needed
                                  // def_item: Command::default(),
        }
    }
    pub fn set_def_idx(&mut self, new_idx: usize) -> &mut State {
        self.def_idx = new_idx;
        return self;
    }
    pub fn set_cur_idx(&mut self, new_idx: usize) -> &mut State {
        self.cur_idx = new_idx;
        return self;
    }
    pub fn set_cmd_loc(&mut self, new_loc: usize) -> &mut State {
        self.cmd_loc = new_loc;
        return self;
    }
    pub fn set_start(&mut self, new_start: usize) -> &mut State {
        self.start = new_start;
        return self;
    }
    pub fn set_end(&mut self, new_end: usize) -> &mut State {
        self.end = new_end;
        return self;
    }
    pub fn set_dmode(&mut self, new_dmode: u8) -> &mut State {
        self.dmode = new_dmode;
        return self;
    }
    pub fn set_is_escaped(&mut self, is_escaped: bool) -> &mut State {
        self.is_escaped = is_escaped;
        return self;
    }
    // pub fn set_def_item(&mut self, new_def_item: Command) -> &mut State {
    //     self.def_item = new_def_item;
    //     return self;
    // }
    pub fn set_callback_idx(&mut self, new_callback_idx: usize) -> &mut State {
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
    pub fn next_def_cmd(&mut self, def_item: Command) -> char {
        return def_item.def.chars().nth(self.def_idx).unwrap();
    }
}

#[derive(Clone, Copy)]
struct CodeCharacter {
    // (char, type, depth, delta)
    pub cur_char: char,
    pub token_type: u8,
    pub depth: usize,
    pub delta: usize,
}

impl CodeCharacter {
    pub fn new(cur_char: char, token_type: u8, depth: usize, delta: usize) -> CodeCharacter {
        return CodeCharacter {
            cur_char,
            token_type,
            depth,
            delta,
        };
    }
    pub fn depth_plus_delta(self) -> usize {
        return self.depth + self.delta;
    }
}

#[derive(Debug, Clone, Copy)]
struct Paren {
    pub start: usize,
    pub end: usize,
    pub cmd_loc: usize,
    pub marker: bool,
}

impl Paren {
    pub fn new(start: usize, end: usize, cmd_loc: usize, marker: bool) -> Paren {
        return Paren {
            start,
            end,
            cmd_loc,
            marker,
        };
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Command {
    pub name: String,
    pub default: String,
    pub def: String,
}

impl Command {
    pub fn default() -> Command {
        return Command::new(String::from(""), String::from(""), String::from(""));
    }
    pub fn new(name: String, default: String, def: String) -> Command {
        return Command { name, default, def };
    }
}

#[derive(Debug, Clone)]
struct StateHolder {
    states: Vec<State>,
}
impl StateHolder {
    pub fn push_state(&mut self, new_state: State) {
        self.states.push(new_state);
    }
    pub fn safe_push_state(&mut self, new_state: State, correct_len: usize) {
        assert!(self.states.len() == correct_len);
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
        default: String::from("xpw"),
        def: String::from("xpeq"),
    });
    // push a dummy param for all empty ones. just says to eval

    read_file(&mut contents, &mut parens, &mut commands, &mut defs)?;
    // debug
    // output should match hand-written code
    if DEBUG {
        for (i, cur_content_item) in contents.iter().enumerate() {
            print!(
                "c: {},\ti:{}\tt: {},\tD: {},\td: {}\n",
                cur_content_item.cur_char,
                i,
                cur_content_item.token_type,
                cur_content_item.depth,
                cur_content_item.delta
            );
        }
        for (i, cur_paren) in parens.iter().enumerate() {
            print!(
                "idx: {}\t({},\t{})\tcmd: {}\t{}\n",
                i, cur_paren.start, cur_paren.end, cur_paren.cmd_loc, cur_paren.marker
            );
        }
        for (i, cur_command) in commands.iter().enumerate() {
            print!(
                "{}\t|\t{}\t{}\t{}\n",
                i, cur_command.name, cur_command.default, cur_command.def
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
    contents: &mut Vec<CodeCharacter>,
    parens: &mut Vec<Paren>,
    commands: Vec<Command>,
    _defs: String,
) -> String {
    let mut output = String::from("");
    let mut states: StateHolder = StateHolder { states: Vec::new() };
    let mut command_stack: Vec<Paren> = Vec::new();
    let mut cur_state_idx = 0;

    states.push_state(
        *State::new()
            .set_def_idx(0)
            .set_cur_idx(0)
            .set_cmd_loc(parens[0].cmd_loc)
            .set_start(parens[0].start)
            .set_end(parens[0].end)
            .set_dmode(SEARCHING_CMD)
            .set_is_escaped(false)
            .set_callback_idx(0),
    );
    // states.push(State {
    //     def_idx: 0,                 // index within definition of the current command
    //     cur_idx: 0,                 // index within the actual code being iterated on
    //     cmd_loc: parens[0].cmd_loc, // location of command in command array
    //     start: parens[0].start,     // start index of the current parameter
    //     end: parens[0].end,         // index of final char of current param
    //     dmode: SEARCHING_CMD,       // the current mode of the full state
    //     is_escaped: false,          // whether or not the current char is escaped
    //     callback_idx: 0,            // the index of the callback state when needed
    // });

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
            print!("cur state: {:?}\n", cur_state);
            print!(
                "equal: {}\tcur_cmd: {}, \ndef: {:?}\n",
                cur_state.next_def_cmd(commands[cur_state.cmd_loc].clone()) == 'p',
                cur_state.next_def_cmd(commands[cur_state.cmd_loc].clone()),
                commands[cur_state.cmd_loc]
            );

            // match cur_state.next_def_cmd(commands[cur_state.cmd_loc].clone()) {
            let cur_cmd_char = cur_state.next_def_cmd(commands[cur_state.cmd_loc].clone());
            print!("cur_cmd_char: {}, {}\n", cur_cmd_char, cur_cmd_char == 'p');
            print!("cur_cmd_char: {}, {}\n", cur_cmd_char, cur_cmd_char == 'p');
            let is_p = cur_cmd_char == 'p';
            if is_p {
                print!("AAAAAAAAAAAAAAA");
            }
            if cur_cmd_char == 'x' {
                println!("Input is equal to a")
            } else if cur_cmd_char == 'q' {
                unimplemented!()
            } else if cur_cmd_char == 'n' {
                print!("BBB");
                cur_state.jump_to_end();
                cur_state.increment_cur_idx();
                cur_state.dmode = SEARCHING_PARAM;
                continue;
            } else if cur_cmd_char == 'p' {
                print!(
                    "cur depth: {}\tstart depth: {}",
                    contents[cur_state.cur_idx].depth, contents[cur_state.start].depth
                );
                if contents[cur_state.cur_idx].depth < contents[cur_state.start].depth {
                    // if it's inside a param or not
                    // push start and end of param
                    print!(
                        "pushing! {:?}\n",
                        parens[contents[cur_state.cur_idx].depth_plus_delta()]
                    );
                    command_stack.push(parens[contents[cur_state.cur_idx].depth_plus_delta()]);
                    continue;
                } else {
                    cur_state.dmode = SEARCHING_PARAM;
                    continue;
                }
            } else if cur_cmd_char == 's' {
                cur_state.dmode = STRING_WRITING;
                continue;
            } else if cur_cmd_char == 'e' {
                // pop and jump
                print!("popping! {:?}\n", command_stack);
                let cur_cmd = command_stack.pop().unwrap();
                cur_state_idx = contents[cur_cmd.start].depth_plus_delta();
                if states.len() <= cur_state_idx {
                    // if there isn't a state

                    states.safe_push_state(
                        *State::new()
                            .set_def_idx(0)
                            .set_cur_idx(cur_cmd.start)
                            .set_cmd_loc(parens[cur_state_idx].cmd_loc)
                            .set_start(parens[cur_state_idx].start)
                            .set_end(parens[cur_state_idx].end)
                            .set_dmode(SEARCHING_CMD)
                            .set_is_escaped(false)
                            .set_callback_idx(cur_state_idx),
                        cur_state_idx,
                    );
                }
                // pop and jump
                states.get_state(cur_state_idx).set_cur_idx(cur_cmd.start);

                continue;
            } else if cur_cmd_char == 'w' {
                // pop and jump
                let cur_cmd = command_stack.pop().unwrap();
                cur_state_idx = contents[cur_cmd.start].depth_plus_delta();
                if states.len() <= cur_state_idx {
                    // if there isn't a state

                    states.safe_push_state(
                        *State::new()
                            .set_def_idx(0)
                            .set_cur_idx(cur_cmd.start)
                            .set_cmd_loc(parens[cur_state_idx].cmd_loc)
                            .set_start(parens[cur_state_idx].start)
                            .set_end(parens[cur_state_idx].end)
                            .set_dmode(WRITING)
                            .set_is_escaped(false)
                            .set_callback_idx(cur_state_idx),
                        cur_state_idx,
                    );
                }
                // pop and jump
                states.get_state(cur_state_idx).set_cur_idx(cur_cmd.start);

                continue;
            } else {
                print!("AAA");
                unimplemented!();
            }
            // }

            if cur_state.next_def_cmd(commands[cur_state.cmd_loc].clone()) == 'x' {
                continue;
            }
        } else if cur_state.dmode == WRITING {
            let cur_char = contents[cur_state.cur_idx];
            if cur_state.cur_idx == cur_state.start && cur_char.token_type == OPEN_PAREN {
                cur_state.increment_cur_idx();
                continue;
            } else if cur_state.cur_idx == cur_state.end {
                if cur_char.token_type != CLOSE_PAREN {
                    output.push(cur_char.cur_char);
                }
                //recurse up
                cur_state_idx = cur_state.callback_idx;
                continue;
            }
            output.push(cur_char.cur_char);
            cur_state.increment_cur_idx();
            continue;
        }
    }
    return output;
}

fn find_def(defs: String, searchfor: String) -> Command {
    // now find definition
    let mut def_idx: usize = 0; // the position in the definition vec
                                // let mut def_start = 0; // the start of the current definition
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
                return Command::new(cmd, default, def);
                // break;
            }

            // end of def, reset
            // set start to next char
            //         def_start = def_idx + 1;
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
    def.insert_str(0, "x");
    default.insert_str(0, "x");
    return Command::new(cmd, default, def);
}

fn read_file(
    contents: &mut Vec<CodeCharacter>,
    parens: &mut Vec<Paren>,
    commands: &mut Vec<Command>,
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
    let mut last_type;
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
            for (j_rev, cur_paren_item) in parens.iter().rev().enumerate() {
                let marker = cur_paren_item.marker;
                let j = parens.len() - j_rev - 1;
                // print!("backiter, j = {}\n", j);
                // print!("j: {}\tdepth: {}\n", j, depth);
                if marker {
                    continue;
                }
                // adding 1 to account for zero indexing
                // - may remove at some point if it makes things cleaner
                delta = 1 + j - depth;
                break;
            }

            parens[depth + delta - 1].end = i - 1;
            parens[depth + delta - 1].marker = true;
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
            parens.push(Paren::new(i, 0, 0, false));
            delta = parens.len() - depth;
            assert!(parens.len() == (depth + delta));
        } else if c == ')' {
            // move this to next iteration to keep close paren inside
            // the depth, next depth only applies on next iteration
            token_type = CLOSE_PAREN;

            // WATCH OUT, ITER MIGHT BE WRONG, YOU MAY NEED TO DO
            // A len - i INSTEAD
            //find delta with markers

            for (j_rev, cur_paren_item) in parens.iter().rev().enumerate() {
                let marker = cur_paren_item.marker;
                let j = parens.len() - j_rev - 1;
                if marker {
                    continue;
                }
                // adding 1 to account for zero indexing
                // - may remove at some point if it makes things cleaner
                delta = 1 + j - depth;
                break;
            }

            // update parens param
            // print!("depth+delta-1 = {:?}\n", parens[depth + delta - 1]);

            parens[depth + delta - 1].end = i;
            parens[depth + delta - 1].marker = true;
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

                        parens[depth + delta - 1].cmd_loc = commands.len() - 1;
                    } else {
                        // not a new def
                        parens[depth + delta - 1].cmd_loc =
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
            parens.push(Paren::new(i, i, 0, true));
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

            parens[len - 1].end = i;

            token_type = SOLO_PARAM;
        }

        // check for single tokens
        /* print!(
            "c: {}\tlast: {}\ttype: {}\tdepth: {}\tdelta: {}\n",
            c, last_type, token_type, depth, delta
        ); */
        contents.push(CodeCharacter::new(c, token_type, depth, delta));
        if token_type == SOLO_PARAM && (last_type == DELIMINATOR || last_type == SOLO_PARAM) {
            depth -= 1;
        }
    }
    Ok(())
}
