
mod logic;
mod statespace;
mod table;

use logic::{State,Buff,Action,Setting};
use table::*;
use rustyline::Editor;
use rustyline::error::ReadlineError;

#[derive(Debug)]
enum CmdLine {
    Help,
    Exit,
    Empty,
    Other,
    Eval(State),
}

fn parse_eval_state( v:&Vec<&str> ) -> Option<State> {

    if v.len() < 2 {
        None
    }
    else {
        let cp = match v[0].parse::<u16>() {
            Err(_) => return None,
            Ok(x) => x,
        };

        let durability = match v[1].parse::<u8>() {
            Err(_) => return None,
            Ok(x) => x,
        };

        Some( State { cp:cp, durability:durability, buff: Buff { inner_quiet:11, manipulation:0, innovation:0, great_strides:0, waste_not:0, basic_touch:0, observe:0 }} )
    }
}

fn parse_help( v:&Vec<&str> ) -> bool {
    if v.len() < 1 {
        false
    }
    else {
        v[0] == "h" || v[0] == "help"
    }
}

fn parse_exit( v:&Vec<&str> ) -> bool {
    if v.len() < 1 {
        false
    }
    else {
        v[0] == "exit" || v[0] == "quit"
    }
}

fn parse_cmdline( line:&str ) -> CmdLine {
    let v: Vec<&str> = line.split_whitespace().collect();

    if let Some(s) = parse_eval_state(&v) {
        CmdLine::Eval(s)
    }
    else if parse_exit(&v) {
        CmdLine::Exit
    }
    else if parse_help(&v) {
        CmdLine::Help
    }
    else if v.len() == 0 {
        CmdLine::Empty
    }
    else {
        CmdLine::Other
    }
}

fn print_series( setting:&Setting, ta:&Table<Action>, initial_state:&State ) {

    let mut s = *initial_state;
    let mut sum_q = 0;

    while ta[s] != Action::CannotAction {
        let (ns,q) = s.run_action(&setting, &ta[s]);
        sum_q += q;

        println!("{:?} {:?} -> {}(+{})",s,ta[s],sum_q,q);
        s = ns;
    }
}

fn print_help() {
    println!("Usage:");
    println!("  [CP] [durability]    print series for input setting");
    println!("  h, help              print help" );
    println!("  exit, quit           exit command" );
}

fn print_error() {
    println!("Wrong command(h for help)");
}

fn eval_line( setting:&Setting, ta:&Table<Action>, line:&str ) -> bool {

    let cmdline = parse_cmdline( &line );

    match cmdline {
        CmdLine::Eval(s) => print_series(setting,ta,&s),
        CmdLine::Help => print_help(),
        CmdLine::Other => print_error(),
        CmdLine::Empty => return true,
        CmdLine::Exit => return false,
    }

    return true
}

fn repl( setting:&Setting, ta:&Table<Action> ) {

    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if !eval_line( &setting, &ta, &line ) {
                    break;
                }
            },
            Err(ReadlineError::Interrupted) => {
                break;
            },
            Err(ReadlineError::Eof) => {
                break;
            },
            Err(err) => {
                println!("{}", err);
                break;
            }
        }
    }
}

fn main() {
    let setting = Setting {
        max_durability: 55,
        max_cp: 657,
        sustain: false,
        process_accuracy: 2910,
        required_process_accuracy: 2540,
    };

    let (_,ta) = calc_table( &setting );

    repl( &setting, &ta );
}
