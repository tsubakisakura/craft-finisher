
use super::logic::{State,Buff,Action,Setting};
use super::table::*;
use rustyline::Editor;
use rustyline::error::ReadlineError;

#[derive(Debug)]
enum CmdLine {
    Help,
    Exit,
    Empty,
    Eval(State),
}

fn parse_eval( v:&[&str] ) -> Result<CmdLine,&'static str> {

    if v.len() < 2 {
        Err("not enough arguments")
    }
    else {
        let cp = match v[0].parse::<u16>() {
            Err(_) => return Err("cannot parse CP"),
            Ok(x) => x,
        };

        let d = match v[1].parse::<u8>() {
            Err(_) => return Err("cannot parse durability"),
            Ok(x) => x,
        };

        // 耐久は5の倍数切り上げします。
        let durability = ((d+4) / 5 * 5) as u8;

        Ok( CmdLine::Eval( State { cp:cp, durability:durability, buff: Buff { inner_quiet:11, manipulation:0, innovation:0, great_strides:0, waste_not:0, basic_touch:0, observe:0 }} ) )
    }
}

fn is_all_numeric(s:&str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

fn parse_cmdline( line:&str ) -> Result<CmdLine,&'static str> {
    let v: Vec<&str> = line.split_whitespace().collect();

    if v.len() > 0 {
        match v[0] {
            x if is_all_numeric(x) => parse_eval(&v[0..]),
            "eval" => parse_eval(&v[1..]),
            "?" => Ok(CmdLine::Help),
            "h" => Ok(CmdLine::Help),
            "help" => Ok(CmdLine::Help),
            "quit" => Ok(CmdLine::Exit),
            "exit" => Ok(CmdLine::Exit),
            _ => Err("Wrong command (h for help)"),
        }
    }
    else {
        Ok(CmdLine::Empty)
    }
}

fn print_series( setting:&Setting, ta:&Table<Action>, initial_state:&State ) {

    if !ta.contains(initial_state) {
        println!("Out of bound(0<=cp<={} && 5<=durability<={} && durability%5==0)", setting.max_cp, setting.max_durability );
        return;
    }

    let mut s = *initial_state;
    let mut sum_q = 0;

    while ta[s] != Action::CannotAction {
        let (ns,q) = s.run_action(&setting, &ta[s]);
        sum_q += q;

        println!("{:?} {:?} -> {}(+{})",s,ta[s],sum_q,q);
        s = ns;
    }
    println!("{:?}", s);
}

fn print_help() {
    println!("Usage:");
    println!("  [CP] [durability]       print tactics");
    println!("  eval [CP] [durability]  print tactics(same as above)");
    println!("  ?, h, help              print help" );
    println!("  exit, quit              exit command" );
}

fn eval_line( setting:&Setting, ta:&Table<Action>, line:&str ) -> bool {

    let cmdline = parse_cmdline( &line );

    match cmdline {
        Ok(cmd) => match cmd {
            CmdLine::Eval(s) => print_series(setting,ta,&s),
            CmdLine::Help => print_help(),
            CmdLine::Empty => {},
            CmdLine::Exit => return false,
        },
        Err(x) => {
            println!("{}",x)
        },
    }

    return true
}

pub fn repl( setting:&Setting, ta:&Table<Action> ) {

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
