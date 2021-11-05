
use std::collections::HashSet;
use super::logic::{State,Buff};

#[derive(Debug,Clone)]
pub struct StateSpace {
    pub max_durability : u8,        // 最大耐久
    pub max_cp : u16,               // 最大CP
    pub buffs: Vec<Buff>,           // 取りうるバフ一覧
    buff_to_index: Vec<Option<usize>>,       // バフからインデックスへの変換
}

fn travarse_states( states:&mut HashSet<Buff>, s:Buff, sustain:bool ) {
    if states.insert( s.clone() ) {
        let ns = s.next_turn();
        travarse_states( states, Buff{ manipulation:8, ..ns }, sustain );
        travarse_states( states, Buff{ innovation:4, ..ns }, sustain );
        travarse_states( states, Buff{ great_strides:3, ..ns }, sustain );
        travarse_states( states, Buff{ waste_not:4, ..ns }, sustain );
        travarse_states( states, Buff{ waste_not:8, ..ns }, sustain );
        travarse_states( states, Buff{ basic_touch:1, ..ns }, sustain );
        travarse_states( states, Buff{ observe:1, ..ns }, sustain );
        travarse_states( states, Buff{ inner_quiet:0, ..ns }, sustain );

        if sustain {
            travarse_states( states, Buff{ manipulation:10, ..ns }, sustain );
            travarse_states( states, Buff{ innovation:6, ..ns }, sustain );
            travarse_states( states, Buff{ great_strides:5, ..ns }, sustain );
            travarse_states( states, Buff{ waste_not:6, ..ns }, sustain );
            travarse_states( states, Buff{ waste_not:10, ..ns }, sustain );
        }

        travarse_states( states, ns, sustain );
    }
}

fn initial_buff_state() -> Buff {
    Buff {
        inner_quiet:11,
        manipulation:0,
        innovation:0,
        great_strides:0,
        waste_not:0,
        basic_touch:0,
        observe:0,
    }
}

fn generate_buffs( sustain:bool ) -> Vec<Buff> {

    let mut states = HashSet::new();
    travarse_states( &mut states, initial_buff_state(), sustain );
    states.into_iter().collect()
}

impl StateSpace {

    const N_MANIPULATION : usize = 10;
    const N_INNOVATION : usize = 6;
    const N_GREAT_STRIDES : usize = 5;
    const N_WASTE_NOT : usize = 10;
    const N_BASIC_TOUCH : usize = 2;
    const N_OBSERVE : usize = 2;
    const N_INNER_QUIET : usize = 2;

    // 思った以上に表の参照に時間がかかるのでハッシュマップを使わずに直接計算します。
    // 厳密にはsustainの有無でテーブルサイズは変えられますがそこまでメモリを消費しないのでシンプルに定義します
    fn buff_to_addr( s:&Buff ) -> usize {
        let mut x = s.manipulation as usize;
        x *= StateSpace::N_INNOVATION;
        x += s.innovation as usize;
        x *= StateSpace::N_GREAT_STRIDES;
        x += s.great_strides as usize;
        x *= StateSpace::N_WASTE_NOT;
        x += s.waste_not as usize;
        x *= StateSpace::N_BASIC_TOUCH;
        x += s.basic_touch as usize;
        x *= StateSpace::N_OBSERVE;
        x += s.observe as usize;
        x *= StateSpace::N_INNER_QUIET;
        x += s.inner_quiet as usize;
        x
    }

    pub fn new( max_durability: u8, max_cp: u16, sustain: bool ) -> StateSpace {
        let buffs = generate_buffs( sustain );

        let mut buff_to_index = Vec::new();
        buff_to_index.resize( StateSpace::N_MANIPULATION * StateSpace::N_INNOVATION * StateSpace::N_GREAT_STRIDES * StateSpace::N_WASTE_NOT * StateSpace::N_BASIC_TOUCH * StateSpace::N_OBSERVE * StateSpace::N_INNER_QUIET, None );

        for i in 0..buffs.len() {
            let s = buffs[i];
            buff_to_index[StateSpace::buff_to_addr(&s)] = Some(i);
        }

        StateSpace {
            max_durability: max_durability,
            max_cp: max_cp,
            buffs: buffs,
            buff_to_index: buff_to_index,
        }
    }

    pub fn size_cp(&self) -> usize {
        let num_durability = (self.max_durability / 5) as usize;
        let num_buff = self.buffs.len();

        num_durability * num_buff
    }

    pub fn size(&self) -> usize {
        let num_cp = (self.max_cp + 1) as usize;

        num_cp * self.size_cp()
    }

    pub fn get_index(&self, s:&State) -> Option<usize> {
        if s.durability == 0 || s.durability % 5 != 0 || s.durability > self.max_durability {
            None
        }
        else if s.cp > self.max_cp {
            None
        }
        else {
            let cp : usize = s.cp as usize;
            let durability = (s.durability / 5) as usize - 1;
            let num_durability = (self.max_durability / 5) as usize;
            let num_buff = self.buffs.len();

            match self.buff_to_index[StateSpace::buff_to_addr(&s.buff)] {
                Some(i) => Some((cp * num_durability + durability) * num_buff + i),
                None => None,
            }
        }
    }
}
