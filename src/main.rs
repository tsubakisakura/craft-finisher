
mod logic;
mod statespace;

use indicatif::ProgressIterator;
use core::ops::{Index,IndexMut};

use logic::{State,Buff,Action,Setting};
use statespace::{StateSpace};

#[derive(Debug)]
struct Table<T>
{
    values : Vec<T>,
    space : StateSpace,
}

impl<T: Clone> Table<T> {
    fn new( space:&StateSpace, init_value:T ) -> Table<T> {
        let mut values = Vec::new();
        values.resize( space.size(), init_value );

        Table { values: values, space: space.clone() }
    }
}

impl<T: Clone> Index<State> for Table<T> {
    type Output = T;

    fn index(&self, s:State) -> &Self::Output {
        self.values.index(self.space.get_index(&s).unwrap())
    }
}

impl<T: Clone> IndexMut<State> for Table<T> {
    fn index_mut(&mut self, s:State) -> &mut Self::Output {
        self.values.index_mut(self.space.get_index(&s).unwrap())
    }
}

fn calc_value( setting:&Setting, table:&Table<u32>, s:&State ) -> (Action,u32) {
    let mut max_a = Action::CannotAction;
    let mut max_v = 0;

    const CANDIDATE_ACTIONS : [Action;13] = [
        Action::BasicTouch,         // 加工
        Action::StandardTouch,      // 中級加工
        Action::PrudentTouch,       // 倹約加工
        Action::FocusedTouch,       // 注視加工
        Action::PreparatoryTouch,   // 下地加工
        Action::ByregotsBlessing,   // ビエルゴの祝福
        Action::MastersMend,        // マスターズメンド
        Action::Observe,            // 経過観察
        Action::WasteNot,           // 倹約
        Action::WasteNot2,          // 長期倹約
        Action::GreatStrides,       // グレートストライド
        Action::Innovation,         // イノベーション
        Action::Manipulation,       // マニピュレーション
    ];

    for a in CANDIDATE_ACTIONS {
        if s.check_action(&a) {
            let (ns,q) = s.run_action( setting, &a );
            if let Some(index) = table.space.get_index(&ns) {
                let v = q + table.values[index];
                if v > max_v {
                    max_a = a;
                    max_v = v;
                }
            }
        }
    }

    (max_a,max_v)
}

fn calc_table( setting:&Setting ) -> (Table<u32>,Table<Action>) {

    let space = StateSpace::new(setting.max_durability, setting.max_cp, setting.sustain);
    let mut table_v = Table::new( &space, 0 );
    let mut table_a = Table::new( &space, Action::CannotAction );

    // アクションは「何もしない場合」を除いて全てCPを消費するアクションですから、CP順に処理していく必要があります。
    // 「何もしない場合」とは、CP的に実行可能なアクションを実行すると耐久0になる場合しか選択肢がない場合で、この時は報酬は常に0です。
    for cp in (0..=setting.max_cp).progress() {
        for durability in (5..=setting.max_durability).step_by(5) {
            for buff_index in 0..space.buffs.len() {
                let s = State { cp: cp, durability: durability, buff: space.buffs[buff_index] };
                let index = space.get_index(&s).unwrap();

                let (a,v) = calc_value( &setting, &table_v, &s );
                table_v.values[index] = v;
                table_a.values[index] = a;
            }
        }
    }

    (table_v,table_a)
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
