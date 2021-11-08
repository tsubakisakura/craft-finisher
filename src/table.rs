
use indicatif::ProgressIterator;
use rayon::prelude::*;
use core::ops::{Index,IndexMut};

use super::logic::{State,Action,Setting};
use super::statespace::{StateSpace};

#[derive(Debug)]
pub struct Table<T>
{
    values : Vec<T>,
    space : StateSpace,
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

impl<T> Table<T> {
    pub fn contains(&self, s:&State) -> bool {
        self.space.contains(s)
    }
}

fn calc_value( setting:&Setting, values:&[u32], space:&StateSpace, s:&State ) -> (Action,u32) {
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
            if let Some(index) = space.get_index(&ns) {
                let v = q + values[index];
                if v > max_v {
                    max_a = a;
                    max_v = v;
                }
            }
        }
    }

    (max_a,max_v)
}

pub fn calc_table( setting:&Setting ) -> (Table<u32>,Table<Action>) {

    let space = StateSpace::new(setting.max_durability, setting.max_cp, setting.sustain);

    let mut v_buffer = Vec::new();
    let mut a_buffer = Vec::new();
    v_buffer.resize( space.size(), 0 );
    a_buffer.resize( space.size(), Action::CannotAction );

    // CannotActionを除いて全てCPを消費するアクションですから、CP順に処理すれば参照先が未計算ということはないです。
    // CannotActionの場合はどこを参照することもなく単に評価値が0になります。
    for cp in (0..=setting.max_cp).progress() {

        // 計算領域を計算済み領域(v1とa1)と未計算領域(v2とa2)に分割します。
        // 計算済み領域はcp=0で始まる領域で、未計算領域は現イテレーションのcpで始まる領域です。
        // 更に現在の現イテレーションの領域に限定したスライスをvc、ac、と定義します。
        let (v1,v2) = v_buffer.split_at_mut( space.size_cp() * cp as usize );
        let ( _,a2) = a_buffer.split_at_mut( space.size_cp() * cp as usize );
        let vc = &mut v2[0..space.size_cp()];
        let ac = &mut a2[0..space.size_cp()];

        // 現イテレーションの全状態について計算します。
        vc.par_iter_mut().zip(ac.par_iter_mut()).enumerate().chunks(1024).for_each(|slice| {
            for (index,(pv,pa)) in slice {
                let s = space.get_state_by_cp_index( cp, index );
                let (a,v) = calc_value( &setting, &v1, &space, &s );
                *pv = v;
                *pa = a;
            }
        });
    }

    (Table { values:v_buffer, space:space.clone() },
     Table { values:a_buffer, space:space.clone() } )
}
