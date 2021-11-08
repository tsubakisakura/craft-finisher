
mod logic;
mod statespace;
mod table;
mod repl;

use logic::Setting;
use table::*;
use repl::*;

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
