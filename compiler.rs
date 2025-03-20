#!allow(warnings)]
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
const FILENAME: &str = "test.gl";

/*
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

const DEBUG: bool = false;

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
    pub fn decrement_def_idx(&mut self) {
        self.def_idx -= 1;
    }
    pub fn increment_cur_idx(&mut self) {
        self.cur_idx += 1;
    }
    // pub fn increment_cmd_loc(&mut self) {
    //     self.cmd_loc += 1;
    // }
    // pub fn jump_to_end(&mut self) {
    //     self.cur_idx = self.end;
    // }
    // pub fn jump_to_start(&mut self) {
    //     self.cur_idx = self.start;
    // }
    // TODO: make it smarter and store internally(?)
    pub fn next_def_cmd(&mut self, def_item: Command) -> char {
        let def_chars = def_item.clone().def;
        // println!("def_idx: {}, def:{:?}", self.def_idx, def_item);
        if def_chars.len() > self.def_idx {
            return def_chars.chars().nth(self.def_idx).unwrap();
        }
        // println!("going to default cmd");
        // if it's past the def, go to the default
        let default_chars = def_item.clone().default;
        let idx = (self.def_idx - def_chars.len()) % default_chars.len();
        return default_chars.chars().nth(idx).unwrap();
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
    // WARNING: This will be minus one to be used as a value. It may
    // not behave as expected
    pub fn depth_plus_delta(self) -> usize {
        return self.depth + self.delta - 1;
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
    // pub fn default() -> Command {
    //     return Command::new(String::from(""), String::from(""), String::from(""));
    // }
    pub fn new(name: String, default: String, def: String) -> Command {
        return Command { name, default, def };
    }
}

#[derive(Debug, Clone)]
struct StateHolder {
    pub states: Vec<State>,
}
impl StateHolder {
    pub fn push_state(&mut self, new_state: State) {
        self.states.push(new_state);
    }
    pub fn safe_push_state(&mut self, new_state: State, correct_len: usize) {
        assert!(self.states.len() == correct_len);
        self.states.push(new_state);
    }
    pub fn safe_set_state(&mut self, new_state: State, correct_idx: usize) {
        assert!(self.states.len() > correct_idx);
        self.states[correct_idx] = new_state;
    }
    pub fn get_state(&mut self, idx: usize) -> &mut State {
        return &mut self.states[idx];
    }
    pub fn _len(state: &StateHolder) -> usize {
        return state.clone().states.len();
    }
}

// WARNING:
// All of this macro code was written at about 2AM, it's really bad
#[derive(Clone, Debug)]
struct MacroHolder {
    pub macros: HashMap<String, BytecodeMacro>,
}

impl MacroHolder {
    // tests if a string is a defined macro
    pub fn new() -> MacroHolder {
        let holder = MacroHolder {
            macros: std::collections::HashMap::new(),
        };
        // deal with empty strings
        // holder.add_macro(String::from(""), Vec::new(), String::from(""));
        return holder;
    }
    pub fn is_macro(&self, name: String) -> bool {
        return self.macros.contains_key(name.as_str());
    }
    pub fn get_def(&self, name: String) -> String {
        // eprint!("NAME: {}", name);
        return self.macros.get(name.as_str()).unwrap().return_definition();
    }
    pub fn add_macro(&mut self, name: String, params: Vec<String>, def: String) {
        self.macros
            .insert(name.clone(), BytecodeMacro::new(name.clone(), params, def));
    }
    pub fn replace_params(&self, def: String, params: Vec<String>) -> String {
        let out = def;
        for param in params {
            // println!("param to replace: {}", param);
        }
        return out;
    }
    pub fn expand_macros(&self, name: String, params: Vec<String>) -> String {
        let def = self.get_def(name);
        return self.replace_params(def, params).to_string();
        // return self.macros.get(name).unwrap().to_string();
    }
}

#[derive(Clone, Debug)]
struct BytecodeMacro {
    pub name: String,
    pub params: Vec<String>,
    pub def: String,
}

impl BytecodeMacro {
    pub fn new(name: String, params: Vec<String>, def: String) -> BytecodeMacro {
        return BytecodeMacro { name, params, def };
    }
    pub fn return_definition(&self) -> String {
        return self.def.clone();
    }
}

#[derive(Debug, Clone)]
enum BytecodeInstruction {
    Init { n: usize },
    NewVar { var_name: String },
    Jta { array_name: String },
    Jtv { var_name: String },
    Jtc { var_name: String },
    NextVar,
    PrevVar,
    NextArr,
    PrevArr,
    NextChar,
    PrevChar,
    While { loopname: String },
    EndWhile { loopname: String },
    Load,
    Swap,
    Sub,
    Add,
    Put,
    AddC { c: usize },
    SubC { c: usize },
    Set { c: usize },
    Clear,
    Printc,
    Printi,
    Inputc,
    Error,
    End,
}
impl BytecodeInstruction {
    pub fn get_instruction(instruction_name: &str, params: Vec<String>) -> Option<Self> {
        match instruction_name {
            "init" => {
                if params.len() == 1 {
                    if let Ok(n) = params[0].parse::<usize>() {
                        Some(BytecodeInstruction::Init { n })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "newvar" => {
                if params.len() == 1 {
                    Some(BytecodeInstruction::NewVar {
                        var_name: params[0].clone(),
                    })
                } else {
                    None
                }
            }
            "jta" => {
                if params.len() == 1 {
                    Some(BytecodeInstruction::Jta {
                        array_name: params[0].clone(),
                    })
                } else {
                    None
                }
            }
            "jtv" => {
                if params.len() == 1 {
                    Some(BytecodeInstruction::Jtv {
                        var_name: params[0].clone(),
                    })
                } else {
                    None
                }
            }
            "jtc" => {
                if params.len() == 1 {
                    Some(BytecodeInstruction::Jtc {
                        var_name: params[0].clone(),
                    })
                } else {
                    None
                }
            }
            "while" => {
                if params.len() == 1 {
                    let Ok(loopname) = params[0].parse::<String>();
                    Some(BytecodeInstruction::While { loopname })
                } else {
                    None
                }
            }
            "addc" => {
                if params.len() == 1 {
                    if let Ok(c) = params[0].parse::<usize>() {
                        Some(BytecodeInstruction::AddC { c })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "subc" => {
                if params.len() == 1 {
                    if let Ok(c) = params[0].parse::<usize>() {
                        Some(BytecodeInstruction::SubC { c })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "set" => {
                if params.len() == 1 {
                    if let Ok(c) = params[0].parse::<usize>() {
                        Some(BytecodeInstruction::Set { c })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "clear" => {
                if params.is_empty() {
                    Some(BytecodeInstruction::Clear)
                } else {
                    None
                }
            }
            "printc" => {
                if params.join("") == "" {
                    Some(BytecodeInstruction::Printc)
                } else {
                    None
                }
            }
            "printi" => {
                if params.join("") == "" {
                    Some(BytecodeInstruction::Printi)
                } else {
                    None
                }
            }
            "inputc" => {
                if params.is_empty() {
                    Some(BytecodeInstruction::Inputc)
                } else {
                    None
                }
            }
            "error" => {
                if params.is_empty() {
                    Some(BytecodeInstruction::Error)
                } else {
                    None
                }
            }
            "end" => {
                if params.is_empty() {
                    Some(BytecodeInstruction::End)
                } else {
                    None
                }
            }
            // For instructions without parameters
            "nextvar" => Some(BytecodeInstruction::NextVar),
            "prevvar" => Some(BytecodeInstruction::PrevVar),
            "nextarr" => Some(BytecodeInstruction::NextArr),
            "prevarr" => Some(BytecodeInstruction::PrevArr),
            "nextchar" => Some(BytecodeInstruction::NextChar),
            "prevchar" => Some(BytecodeInstruction::PrevChar),
            "endwhile" => {
                if params.len() == 1 {
                    let Ok(loopname) = params[0].parse::<String>();
                    Some(BytecodeInstruction::While { loopname })
                } else {
                    None
                }
            }
            "load" => Some(BytecodeInstruction::Load),
            "swap" => Some(BytecodeInstruction::Swap),
            "sub" => Some(BytecodeInstruction::Sub),
            "add" => Some(BytecodeInstruction::Add),
            "put" => Some(BytecodeInstruction::Put),
            _ => None,
        }
    }
    pub fn get_wsa_definition(&self, vars: VarsHolder) -> String {
        match self {
            BytecodeInstruction::Init { n } => {
                format!("push -1\npush {}\nstore\npush 0\ncall set_i\npush 1\ncall set_j\npush 0\ncall set_k\ncall set_x\n", n)
            }
            BytecodeInstruction::NewVar { var_name } => {
                // format!("declare variable {}", vars.get_var(var_name))
                format!("# making var {}\n", var_name)
            }
            BytecodeInstruction::Jta { array_name } => {
                // the formatting is done in the VarsHolder
                format!("{}\n", vars.get_pos_set(array_name))
            }
            BytecodeInstruction::Jtv { var_name } => {
                // the formatting is done in the VarsHolder
                format!("{}\n", vars.get_pos_set(var_name))
            }
            BytecodeInstruction::Jtc { var_name } => {
                // the formatting is done in the VarsHolder
                format!("{}\n", vars.get_pos_set(var_name))
            }
            BytecodeInstruction::NextVar => "call inc_j".to_string(),
            BytecodeInstruction::PrevVar => "call dec_j".to_string(),
            BytecodeInstruction::NextArr => "call inc_i".to_string(),
            BytecodeInstruction::PrevArr => "call dec_i".to_string(),
            BytecodeInstruction::NextChar => "call inc_k".to_string(),
            BytecodeInstruction::PrevChar => "call dec_k".to_string(),
            BytecodeInstruction::While { loopname } => {
                // the formatting is done in the VarsHolder
                format!("mul -1\nlabel {}\nmul -1\n", loopname)
            }
            BytecodeInstruction::EndWhile { loopname } => {
                // the formatting is done in the VarsHolder
                format!("dup\nmul -1\n jn {}\nret\n", loopname)
            }
            // BytecodeInstruction::EndWhile => "end while loop".to_string(),
            BytecodeInstruction::Load => "call get_val".to_string(),
            BytecodeInstruction::Swap => "swap".to_string(),
            BytecodeInstruction::Sub => "copy 1\nsub\n".to_string(),
            BytecodeInstruction::Add => "copy 1\nadd\n".to_string(),
            BytecodeInstruction::Put => {
                "call get_k\ncall get_val\ncall set_val\ncall get_x\nstore\ndrop".to_string()
            }
            BytecodeInstruction::AddC { c } => {
                format!("add {}\n", c)
            }
            BytecodeInstruction::SubC { c } => {
                format!("sub {}\n", c)
            }
            BytecodeInstruction::Set { c } => {
                format!("drop\npush {}\n", c)
            }
            BytecodeInstruction::Clear => "drop\npush 0\n".to_string(),
            BytecodeInstruction::Printc => "dup\nprintc\n".to_string(),
            BytecodeInstruction::Printi => "dup\nprinti\n".to_string(),
            BytecodeInstruction::Inputc => "push -8\nreadc\npush -8\nretrieve\n".to_string(),
            BytecodeInstruction::Error => "call error_handler\n".to_string(),
            BytecodeInstruction::End => "end\n".to_string(),
        }
    }
    pub fn replace_wsa_lib(raw: String) -> String {
        let mut output = raw;
        let mut lib = HashMap::new();
        // copy 1 might not work.... We'll see
        let libkeys = [
            "set_i",
            "set_j",
            "set_k",
            "set_x",
            "get_x",
            "get_i",
            "get_i_last",
            "get_j",
            "get_j_last",
            "get_k",
            "get_k_last",
            "get_N",
            "inc_i",
            "inc_j",
            "inc_k",
            "dec_i",
            "dec_j",
            "dec_k",
            "get_val",
            "set_val",
        ];
        lib.insert(
            "set_i",
            "push -5\npush -2\nretrieve\nstore\npush -2\ncopy 1\nstore\nret\n",
        );
        lib.insert(
            "set_j",
            "push -6\npush -3\nretrieve\nstore\npush -3\ncopy 1\nstore\nret\n",
        );
        lib.insert(
            "set_k",
            "push -7\npush -4\nretrieve\nstore\npush -4\ncopy 1\nstore\nret\n",
        );
        lib.insert(
            "set_x",
            "push -2\nretrieve\npush -3\nretrieve\npush -1\nretrieve\nmul\nadd\nret\n",
        );
        lib.insert("get_i", "push -2\nretrieve\nret\n");
        lib.insert(
            "get_x",
            "call get_j\ncall get_N\nmul\ncall get_i\nadd\nret\n",
        );
        lib.insert("get_i_last", "push -5\nretrieve\nret\n");
        lib.insert("get_j", "push -3\nretrieve\nret\n");
        lib.insert("get_j_last", "push -6\nretrieve\nret\n");
        lib.insert("get_k", "push -4\nretrieve\nret\n");
        lib.insert("get_k_last", "push -7\nretrieve\nret\n");
        lib.insert("get_N", "push -1\nretrieve\nret\n");
        lib.insert(
            "inc_i",
            "push -5\npush -2\nretrieve\nstore\npush -2\npush -2\npush 1\nadd\nstore\nret\n",
        );
        lib.insert(
            "inc_j",
            "push -6\npush -3\nretrieve\nstore\npush -3\npush -3\npush 1\nadd\nstore\nret\n",
        );
        lib.insert(
            "inc_k",
            "push -7\npush -4\nretrieve\nstore\npush -4\npush -4\npush 1\nadd\nstore\nret\n",
        );
        lib.insert(
            "dec_i",
            "push -5\npush -2\nretrieve\nstore\npush -2\npush -2\npush 1\nsub\nstore\nret\n",
        );
        lib.insert(
            "dec_j",
            "push -6\npush -3\nretrieve\nstore\npush -3\npush -3\npush 1\nsub\nstore\nret\n",
        );
        lib.insert(
            "dec_k",
            "push -7\npush -4\nretrieve\nstore\npush -4\npush -4\npush 1\nsub\nstore\nret\n",
        );
        lib.insert("get_val", "push -4\nretrieve\nretrieve\nswap\nlabel get_val_loop\ndup\njz get_val_end\nsub 1\nswap\ndiv 256\nswap\njmp get_val_loop\nlabel get_val_end\nswap\nmod 256\nret\n");

        // This thing took me 5 HOURS!!! (And I still got it wrong...l
        lib.insert("set_val", "dup\npush -12\nswap\nstore\npush -13\npush -4\nretrieve\nstore\nretrieve\npush -10\npush 1\nstore\npush -11\npush 0\nstore\ncall set_value\nret\nlabel set_value\nmul -1\ncall unroll_counter\npush -12\npush 19 store\npush -13\npush 1\nstore\ncall reroll\nret\nlabel unroll_counter\nmul -1\ndup\nmod 256\nswap\npush -11\nretrieve\nadd 1\npush -11\nswap\nstore\ndiv 256\nmul -1\ndup\njn unroll_counter\nret\nlabel reroll\npush -13\nretrieve\njz change_val\njmp change_val_end\nlabel change_val\ndrop push -12\nretrieve\nlabel change_val_end\npush -13\nretrieve\nsub 1\npush -13\nswap\nstore\nmul 256\nadd\npush -11\nretrieve\nsub 1\npush -11\nswap\nstore\npush -11\nretrieve\nmul -1\njn reroll\nret\n");

        for key in libkeys {
            let def = lib.get(key).unwrap();
            // eprintln!("item, def: {}", key);
            // output = output.replace(item, (String::from("call ") + item).as_str());
            // output.push_str("label " + item + "\n" + def);
            output.push_str("end\nlabel ");
            output.push_str(key);
            output.push_str("\n");
            output.push_str(def);
            output.push_str("\n");
        }

        return output;
    }
    pub fn is_instruction(test: &str) -> bool {
        return [
            "macro", "endmacro", "init", "newvar", "jta", "jtv", "jtc", "nextvar", "nextarr",
            "nextchar", "prevvar", "prevarr", "prevchar", "while", "end", "load", "swap", "sub",
            "add", "put", "addc", "subc", "set", "clear", "printc", "printi", "inputc", "error",
            "exit",
        ]
        .contains(&test);
    }
}

#[derive(Debug, Clone)]
pub struct VarsHolder {
    vars_locs: HashMap<String, (u16, u16, u16)>,
    vars: Vec<String>,
}

impl VarsHolder {
    pub fn new() -> VarsHolder {
        return VarsHolder {
            vars_locs: HashMap::new(),
            vars: Vec::new(),
        };
    }
    pub fn add_var(&mut self, varname: &str, loc: (u16, u16, u16)) {
        if self.vars_locs.contains_key(varname) {
            return;
        }
        self.vars_locs.insert(varname.to_string(), loc);
        self.vars.push(varname.to_string());
    }
    pub fn get_var(&self, varname: &str) -> String {
        // println!("getting var for {}", varname);
        return VarsHolder::merge_tuple(self.get_loc(varname)).to_string();
    }

    pub fn has_var(&self, varname: &str) -> bool {
        return self.vars_locs.contains_key(varname);
    }

    pub fn get_loc(&self, varname: &str) -> (u16, u16, u16) {
        // println!("getting var for {}", varname);
        return *self.vars_locs.get(varname).unwrap();
    }
    pub fn merge_tuple(tuples: (u16, u16, u16)) -> String {
        let (a, b, c) = tuples;
        format!("{} {} {}", a, b, c)
    }
    pub fn get_pos_set(&self, varname: &str) -> String {
        let (a, b, c) = *self.vars_locs.get(varname).unwrap();
        format!(
            "push {}\ncall set_i\npush {}\ncall set_j\npush {}\ncall set_k\ncall set_x\n",
            a, b, c
        )
    }
}

#[derive(Debug, Clone)]
pub struct Bytecode {
    instructions: Vec<BytecodeInstruction>,
    macros: MacroHolder,
    // name, (params, def)
    temp_macros: HashMap<String, (Vec<String>, String)>,
    // name
    cur_searching: Vec<String>,
    vars: VarsHolder,
    cur_loc: (u16, u16, u16),
}

impl Bytecode {
    pub fn new() -> Self {
        Bytecode {
            instructions: Vec::new(),
            macros: MacroHolder::new(),
            temp_macros: HashMap::new(),
            cur_searching: Vec::new(),
            cur_loc: (0, 1, 0),
            vars: VarsHolder::new(),
        }
    }

    pub fn input_raw_commands(&mut self, combined: &str) {
        for split in combined.split(['\n', ';']) {
            self.process_cmd(split);
        }
    }

    pub fn generate_wsa(&self) -> String {
        let mut output = String::from("");
        for instruction in self.instructions.clone().iter() {
            output.push_str(instruction.get_wsa_definition(self.vars.clone()).as_str());
            output.push('\n');
        }
        output = BytecodeInstruction::replace_wsa_lib(output);
        return output;
    }

    pub fn process_cmd(&mut self, line: &str) {
        // println!("processing cmd: {}", line);
        if line == "" {
            // blank line, ignore
            return;
        }
        let mut params = Vec::new();
        let mut command = String::from("");
        for param in line.split([' ', '\t']) {
            if command == "" {
                // println!("command: {}", param);
                command = String::from(param);
            } else {
                // println!("param: {}", param);
                params.push(String::from(param));
            }
        }
        // println!("command: \"{}\", params: {:?}", command, params);
        if BytecodeInstruction::is_instruction(command.as_str()) {
            if DEBUG {
                println!("is instruction: {} {:?}", command.as_str(), params);
            }
            let mut to_remove = (false, 0);
            // add macro definition if macro
            if command == "macro" {
                let macro_name = params[0].clone();
                // println!("started macro def (1)");
                // params.remove(0);
                self.temp_macros
                    .insert(macro_name.clone(), (params, String::from("")));
                self.cur_searching.push(macro_name);
            } else if self.cur_searching.len() > 0 {
                // let params[0].clone() = params[0].clone();
                // println!("in macro def (2)");
                for (i, name) in self.cur_searching.iter().enumerate() {
                    if command == "endmacro" && *name == params[0].clone() {
                        // end of macro def
                        to_remove = (true, i);
                        // println!("found end of macro def for {}", params[0].clone());
                        let mut params_minus_one = params.clone();
                        params_minus_one.remove(0);
                        self.macros.add_macro(
                            name.clone(),
                            params_minus_one,
                            self.temp_macros.get(name).unwrap().1.clone(),
                        );
                    } else {
                        // println!("name: {}, temp macros: {:?}", name, self.temp_macros);
                        self.temp_macros
                            .get_mut(name)
                            .unwrap()
                            .1
                            .push_str(&(String::from("\n") + &command + " " + &params.join(" ")));
                    }
                }
            } else {
                if params.len() > 0 {
                    let label = params[0].as_str();
                    if self.vars.has_var(label) {
                        self.cur_loc = self.vars.get_loc(label);
                    } else if command == "newvar" {
                        self.cur_loc.1 += 1;
                        self.vars.add_var(label, self.cur_loc);
                        // increment variable idx
                    } else if command == "jta" {
                        self.vars.add_var(label, self.cur_loc);
                        self.cur_loc.0 += 1;
                    } else if command == "jtc" {
                        self.cur_loc.2 += 1;
                        self.vars.add_var(label, self.cur_loc);
                    }
                }
                // println!("EVALUATING THIS: {}", command);
                // println!(
                //     "instruction: {:#?}",
                //     BytecodeInstruction::get_instruction(command.as_str(), params).unwrap()
                // );
                if command.as_str() != "" {
                    let instruction =
                        BytecodeInstruction::get_instruction(command.as_str(), params).unwrap();
                    self.add_instruction(instruction);
                }
            }
            if to_remove.0 {
                self.temp_macros
                    .remove(self.cur_searching.get(to_remove.1).unwrap());
                self.cur_searching.remove(to_remove.1);
            }
        } else if self.macros.is_macro(command.to_string()) {
            // println!("eval macro def (3)");
            // println!("is macro! {}", command);
            self.input_raw_commands(self.macros.expand_macros(command, params).as_str());
        } else {
            // println!("other (4)");
            // println!("macro usage! '{}'\n{:#?}", command, self.macros);
            let def = self.macros.get_def(command);
            let new_def = self.macros.replace_params(def, params);
            // println!("new def: {}", new_def);
        }
    }

    pub fn get_instructions(self, command: &str, args: Vec<String>) {
        if self.macros.is_macro(command.to_string()) {}
    }
    pub fn add_instruction(&mut self, instruction: BytecodeInstruction) {
        self.instructions.push(instruction);
    }

    // pub fn add_macro_definition(
    //     &mut self,
    //     name: String,
    //     params: Vec<String>,
    //     body: String,
    // ) {

    //     self.macros.insert(name, macro_def);
    // }

    pub fn resolve_macros(&mut self) {}
}

struct Macro {
    pub label: String,
    pub id: i64,
    pub replacement: String,
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
        default: String::from("xq"),
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

    let bytecode = make_bytecode(&mut contents, &mut parens, commands, defs);
    let mut macro_replaced_bytecode = Bytecode::new();
    macro_replaced_bytecode.input_raw_commands(bytecode.as_str());

    let generated_bytecode = macro_replaced_bytecode.generate_wsa();
    // println!("bytecode stuff: {:#?}", macro_replaced_bytecode);
    println!(";generated bytecode: \n;===\n{}\n;===", generated_bytecode);
    if DEBUG {
        eprintln!("macro_replaced: {:?}", macro_replaced_bytecode);
        print!("Got bytecode!\n===\n{}\n===\n", bytecode.clone());
    }
    // let whitespace = compile_whitespace(bytecode);
    // let brainfck = compile_brainfck(bytecode);
    Ok(())
}

fn load_defs(defs: &mut String) {
    // evaluation thingy
    defs.push_str(r#"Enxnpepeq"#);
    //while loop definition
    defs.push_str(r#"wnxnppes\\nwhile\\n npes\\n es\\nend\\n q"#);
    //init command definition
    defs.push_str(r#"Nnxnsinit\\  pws\\n q"#);
    //comments
    defs.push_str("cnxnxq");
    // addition
    defs.push_str("+nxnpnpees\nadd q");
    // addition
    defs.push_str(r#"c+nxnpes\\naddc\\  npws\\n q"#);
    defs.push_str(r#"$nxns\\njtv\\  pws\\nload\\n q"#);
    // subtraction
    defs.push_str("-nxnpnpees\nadd q");
    defs.push_str(r#"\newvarnxns\\nnewvar\  pwq"#);
    defs.push_str("bcnpwnpwq");
    defs.push_str("testnxnsa pwnpwq");
    // print!("defs: {}", defs);
}

fn _preprocess_bytecode(bytecode: String) {
    // now time to actually get working...
    let mut macros: Vec<BytecodeMacro> = Vec::new();
}

fn compile_whitespace(_bytecode: String) {}
fn compile_brainfck(_bytecode: String) {}

fn make_bytecode(
    contents: &mut Vec<CodeCharacter>,
    parens: &mut Vec<Paren>,
    commands: Vec<Command>,
    _defs: String,
) -> String {
    let mut output = String::from("");
    let mut states: StateHolder = StateHolder { states: Vec::new() };
    let mut command_stack: Vec<(Paren, usize)> = Vec::new();
    let mut cur_state_idx = 0;
    // push to the stack
    let mut to_push: (Option<State>, usize) = (None, 42);
    // set a specified value on a stack
    let mut to_set: (Option<State>, usize) = (None, 42);
    let mut states_len = 1;

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

    // init
    let mut cur_idx = 0;
    // println!("starting init...");
    loop {
        // println!("idx: {}", cur_idx);
        if cur_idx >= contents.len() {
            break;
        }
        let cur_char = contents[cur_idx].depth_plus_delta();
        if cur_char >= states_len {
            states.push_state(
                *State::new()
                    .set_def_idx(0)
                    .set_cur_idx(cur_idx)
                    .set_cmd_loc(parens[cur_char].cmd_loc)
                    .set_start(parens[cur_char].start)
                    .set_end(parens[cur_char].end)
                    .set_dmode(SEARCHING_CMD)
                    .set_is_escaped(false)
                    .set_callback_idx(states_len - 1),
            );
            states_len += 1;
        }
        cur_idx += 1;
    }

    // this is a rewrite based on a tree diagram I wrote
    loop {
        // print!("\n");
        if to_push.0.is_some() {
            states.safe_push_state(to_push.0.unwrap(), to_push.1);
            to_push = (None, 42);
            states_len += 1;
            continue;
        }
        if to_set.0.is_some() {
            states.safe_set_state(to_set.0.unwrap(), to_set.1);
            to_set = (None, 42);
            continue;
        }
        let cur_state = states.get_state(cur_state_idx);
        if cur_state.cur_idx >= contents.len() {
            break;
        }
        if cur_state.cur_idx > cur_state.end {
            // if it's the end of the state, break
            // println!(
            //     "callback. cur_idx: {}\tend: {}, cur_state_idx: {}, callback: {}",
            //     cur_state.cur_idx, cur_state.end, cur_state_idx, cur_state.callback_idx
            // );
            if cur_state_idx == 0 {
                break;
            }
            cur_state_idx = cur_state.callback_idx;
            continue;
        }
        if cur_state.dmode == SEARCHING_CMD {
            // print!("searching cmd");
            cur_state.increment_cur_idx();
            if contents[cur_state.cur_idx].token_type == COMMAND {
                cur_state.dmode = EVALUATING;

                // push state if it's not there
                let old_state_idx = cur_state_idx;
                cur_state_idx = contents[parens[cur_state_idx].start].depth_plus_delta();
                if states_len <= cur_state_idx {
                    // if there isn't a state

                    // println!("states.len: {}, idx: {}", states_len, cur_state_idx);
                    to_push = (
                        Some(
                            *State::new()
                                .set_def_idx(0)
                                .set_cur_idx(cur_state.cur_idx)
                                .set_cmd_loc(parens[cur_state_idx].cmd_loc)
                                .set_start(parens[cur_state_idx].start)
                                .set_end(parens[cur_state_idx].end)
                                .set_dmode(SEARCHING_CMD)
                                .set_is_escaped(false)
                                .set_callback_idx(old_state_idx),
                        ),
                        cur_state_idx,
                    );
                }
            }
            // go back to next state
            continue;
        } else if cur_state.dmode == EVALUATING {
            cur_state.increment_def_idx();
            // print!(
            //     "cmd: {} idx: {}\tcur state: {:?}\n",
            //     cur_state.next_def_cmd(commands[cur_state.cmd_loc].clone()),
            //     cur_state_idx,
            //     cur_state,
            // );

            // match cur_state.next_def_cmd(commands[cur_state.cmd_loc].clone()) {
            let cur_cmd_char = cur_state.next_def_cmd(commands[cur_state.cmd_loc].clone());
            if cur_cmd_char == 'x' {
                // println!("Input is equal to a");
                continue;
            } else if cur_cmd_char == 'q' {
                if cur_state_idx == 0 {
                    break;
                }
                cur_state_idx = cur_state.callback_idx;
                // unimplemented!();
                continue;
            } else if cur_cmd_char == 'n' {
                // cur_state.jump_to_end();
                cur_state.set_cur_idx(parens[contents[cur_state.cur_idx].depth_plus_delta()].end);
                cur_state.increment_cur_idx();
                cur_state.increment_def_idx();
                cur_state.dmode = SEARCHING_PARAM;

                continue;
            } else if cur_cmd_char == 'p' {
                // print!(
                //     "cur depth: {}\tstart depth: {}\n",
                //     contents[cur_state.cur_idx].depth, contents[cur_state.start].depth
                // );
                if contents[cur_state.cur_idx].depth > contents[cur_state.start].depth {
                    // if it's inside a param or not
                    // push start and end of param
                    // eprint!(
                    //     "pushing! {:?}\n",
                    //     parens[contents[cur_state.cur_idx].depth_plus_delta()]
                    // );
                    command_stack.push((
                        parens[contents[cur_state.cur_idx].depth_plus_delta()],
                        cur_state_idx,
                    ));
                    continue;
                } else {
                    cur_state.dmode = SEARCHING_PARAM;
                    continue;
                }
            } else if cur_cmd_char == 's' {
                cur_state.dmode = STRING_WRITING;
                cur_state.increment_def_idx();
                // println!("String writing");
                continue;
            } else if cur_cmd_char == 'e' {
                // pop and jump
                // print!("popping! {:?}\n", command_stack);
                let (cur_cmd, callback_idx) = command_stack.pop().unwrap();

                cur_state_idx = contents[cur_cmd.start].depth_plus_delta();
                if states_len <= cur_state_idx {
                    // if there isn't a state

                    // println!("states.len: {}, idx: {}", states_len, cur_state_idx);
                    to_push = (
                        Some(
                            *State::new()
                                .set_def_idx(0)
                                .set_cur_idx(cur_cmd.start)
                                .set_cmd_loc(parens[cur_state_idx].cmd_loc)
                                .set_start(parens[cur_state_idx].start)
                                .set_end(parens[cur_state_idx].end)
                                .set_dmode(SEARCHING_CMD)
                                .set_is_escaped(false)
                                .set_callback_idx(callback_idx),
                        ),
                        cur_state_idx,
                    );
                    // println!("topush: {:#?}", to_push);
                    continue;
                }
                // pop and jump
                states.get_state(cur_state_idx).set_cur_idx(cur_cmd.start);

                continue;
            } else if cur_cmd_char == 'w' {
                // pop and jump
                // println!("command: {:?}", commands[cur_state.cmd_loc]);
                let (cur_cmd, callback_idx) = command_stack.pop().unwrap();

                cur_state_idx = contents[cur_cmd.start].depth_plus_delta();
                if states_len <= cur_state_idx {
                    // if there isn't a state

                    to_push = (
                        Some(
                            *State::new()
                                .set_def_idx(0)
                                .set_cur_idx(cur_cmd.start)
                                .set_cmd_loc(parens[cur_state_idx].cmd_loc)
                                .set_start(parens[cur_state_idx].start)
                                .set_end(parens[cur_state_idx].end)
                                .set_dmode(WRITING)
                                .set_is_escaped(false)
                                .set_callback_idx(callback_idx),
                        ),
                        cur_state_idx,
                    );
                    // println!("in writing, pushing: {:#?}", to_push);
                    continue;
                } else {
                    let mut next_state = states.get_state(cur_state_idx).clone();
                    to_set = (Some(*next_state.set_dmode(WRITING)), cur_state_idx);
                }
                // pop and jump
                states.get_state(cur_state_idx).set_cur_idx(cur_cmd.start);

                continue;
            } else {
                print!("AAA dmode: {}\n", cur_cmd_char);
                unimplemented!();
            }
            // }
        } else if cur_state.dmode == WRITING {
            let cur_char = contents[cur_state.cur_idx];
            // println!("Writing, cur char: {}", cur_char.cur_char);
            if cur_state.cur_idx == cur_state.start && cur_char.token_type == OPEN_PAREN {
                cur_state.increment_cur_idx();
                continue;
            } else if cur_state.cur_idx >= cur_state.end {
                if cur_char.token_type != CLOSE_PAREN {
                    output.push(cur_char.cur_char);
                }
                //recurse up
                cur_state.cur_idx = cur_state.start;
                cur_state_idx = cur_state.callback_idx;
                continue;
            }
            output.push(cur_char.cur_char);
            cur_state.increment_cur_idx();
            continue;
        } else if cur_state.dmode == SEARCHING_PARAM {
            // print!(
            //     "searching param\tcur_depth: {}, start_depth: {} \n",
            //     contents[cur_state.cur_idx].depth, contents[cur_state.start].depth
            // );
            if contents[cur_state.cur_idx].depth > contents[cur_state.start].depth {
                // if it's in a parameter
                // fix for the evaluating block incrementing again
                cur_state.decrement_def_idx();
                cur_state.dmode = EVALUATING;
                continue;
            } else if contents[cur_state.cur_idx].depth < contents[cur_state.start].depth {
                // out of param
                cur_state.dmode = SEARCHING_PARAM;
            } else {
                cur_state.increment_cur_idx();
                continue;
            }
        } else if cur_state.dmode == STRING_WRITING {
            let cur_char = cur_state.next_def_cmd(commands[cur_state.cmd_loc].clone());
            // println!(
            //     "String writing: char: {}, is_escaped: {}",
            //     cur_char, cur_state.is_escaped
            // );
            if cur_char == ' ' && !cur_state.is_escaped {
                cur_state.dmode = EVALUATING;
                continue;
            }
            if cur_char == '\\' && !cur_state.is_escaped {
                cur_state.is_escaped = true;
            } else if cur_state.is_escaped {
                // println!("===\nEscaped char: {}\n===", cur_char);
                match cur_char {
                    'n' => output.push('\n'),
                    't' => output.push('\t'),
                    ' ' => output.push(' '),
                    _ => output.push(cur_char),
                }
                cur_state.is_escaped = false;
            } else {
                output.push(cur_char);
            }
            cur_state.increment_def_idx();
            continue;
        } else {
            // println!("cur_state: {:#?}", cur_state);
            unimplemented!();
        }
    }

    return output;
}

fn find_def(defs: String, searchfor: String) -> Command {
    // now find definition
    let mut def_idx: usize = 0; // the position in the definition vec
                                // let mut def_start = 0; // the start of the current definition
    let mut cmd = String::from(""); // the actual command
    let mut default = String::from(""); // the default parameter execution
    let mut def = String::from(""); // the full definition
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
            // def.push('q');
            // print!("reached end, checking...\n");
            // print!(
            //     "is equal: {}\tsearch_for: {}\tcmd: {}\tdef: {}\n",
            //     searchfor == cmd,
            //     searchfor,
            //     cmd,
            //     def
            // );
            // check if it's the correct definition
            if searchfor == cmd {
                // it's the correct command, exit loop, all is well

                def.insert_str(0, "x");
                default.insert_str(0, "x");
                // print!("found correct def! {} {}\n", cmd, def);
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
                default.push('n');
            } else if looking_for == 1 {
                // default.push('n');
                looking_for = 2;
            } else {
                def.push('n');
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
    // print!(
    //     "found def: {:?}\n",
    //     (cmd.clone(), default.clone(), def.clone())
    // );
    if default.clone().is_empty() {
        default = String::from("q");
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
    // print!("reading file!\n");
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
    // print!("code:\n{}\n", tmp);
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
                        // print!(
                        //     "found new command! {}\tputting at {}\n",
                        //     full_command,
                        //     commands.len()
                        // );
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
