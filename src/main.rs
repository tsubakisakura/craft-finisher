
mod logic;
mod statespace;
mod table;

use logic::{State,Buff,Action,Setting};
use table::*;

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

fn main() {
    let setting = Setting {
        max_durability: 55,
        max_cp: 657,
        sustain: false,
        process_accuracy: 2910,
        required_process_accuracy: 2540,
    };

    let (_,ta) = calc_table( &setting );

    // 試しにサンプル系列を表示してみます
    println!("CP74/25");
    print_series( &setting, &ta, &State { durability:25, cp:74, buff: Buff { inner_quiet:11, manipulation:0, innovation:0, great_strides:0, waste_not:0, basic_touch:0, observe:0 }});

    println!("CP110/35");
    print_series( &setting, &ta, &State { durability:35, cp:110, buff: Buff { inner_quiet:11, manipulation:0, innovation:0, great_strides:0, waste_not:0, basic_touch:0, observe:0 }});

    println!("CP131/25");
    print_series( &setting, &ta, &State { durability:25, cp:131, buff: Buff { inner_quiet:11, manipulation:0, innovation:0, great_strides:0, waste_not:0, basic_touch:0, observe:0 }});

    println!("CP146/35");
    print_series( &setting, &ta, &State { durability:35, cp:146, buff: Buff { inner_quiet:11, manipulation:0, innovation:0, great_strides:0, waste_not:0, basic_touch:0, observe:0 }});

    println!("CP242/35");
    print_series( &setting, &ta, &State { durability:35, cp:242, buff: Buff { inner_quiet:11, manipulation:0, innovation:0, great_strides:0, waste_not:0, basic_touch:0, observe:0 }});
}
