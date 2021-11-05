use core::cmp::min;

#[derive(PartialEq,Eq,Hash,Clone,Debug,Copy)]
pub struct Buff {
    pub inner_quiet : u8,
    pub manipulation : u8,
    pub innovation : u8,
    pub great_strides : u8,
    pub waste_not : u8,
    pub basic_touch : u8,
    pub observe : u8,
}

#[derive(PartialEq,Eq,Hash,Clone,Debug,Copy)]
pub struct State {
    pub durability: u8,
    pub cp: u16,
    pub buff: Buff,
}

pub struct Setting {
    pub max_durability: u8,
    pub max_cp: u16,
    pub sustain: bool,
    pub process_accuracy: u32,
    pub required_process_accuracy: u32,
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Action {
    CannotAction,       // 選択肢が他に無い場合
    BasicTouch,         // 加工
    StandardTouch,      // 中級加工
    PrudentTouch,       // 倹約加工
    FocusedTouch,       // 注視加工
    PreparatoryTouch,   // 下地加工
    ByregotsBlessing,   // ビエルゴの祝福
    MastersMend,        // マスターズメンド
    Observe,            // 経過観察
    WasteNot,           // 倹約
    WasteNot2,          // 長期倹約
    GreatStrides,       // グレートストライド
    Innovation,         // イノベーション
    Manipulation,       // マニピュレーション
}

trait ClippedSubtract<T> {
    fn sub_clip(&self,x:T) -> u8;
}

impl ClippedSubtract<u8> for u8 {
    fn sub_clip(&self,x:u8) -> u8 {
        if *self >= x { *self - x } else { 0 }
    }
}

impl Buff {
    pub fn next_turn(&self) -> Buff {
        Buff {
            inner_quiet: self.inner_quiet,
            manipulation: self.manipulation.sub_clip(1),
            innovation: self.innovation.sub_clip(1),
            great_strides: self.great_strides.sub_clip(1),
            waste_not: self.waste_not.sub_clip(1),
            basic_touch: self.basic_touch.sub_clip(1),
            observe: self.observe.sub_clip(1),
        }
    }
}

impl State {
    fn get_required_cp( &self, a:&Action ) -> u16 {
        match a {
            Action::CannotAction => 0,
            Action::BasicTouch => 18,
            Action::StandardTouch => if self.buff.basic_touch > 0 { 18 } else { 32 },
            Action::PrudentTouch => 25,
            Action::FocusedTouch => 18,
            Action::PreparatoryTouch => 40,
            Action::ByregotsBlessing => 24,
            Action::MastersMend => 88,
            Action::Observe => 7,
            Action::WasteNot => 56,
            Action::WasteNot2 => 98,
            Action::GreatStrides => 32,
            Action::Innovation => 18,
            Action::Manipulation => 96,
        }
    }

    fn get_required_durability( &self, a:&Action ) -> u8 {
        let d = match a {
            Action::CannotAction => 0,
            Action::BasicTouch => 10,
            Action::StandardTouch => 10,
            Action::PrudentTouch => 5,
            Action::FocusedTouch => 10,
            Action::PreparatoryTouch => 20,
            Action::ByregotsBlessing => 10,
            Action::MastersMend => 0,
            Action::Observe => 0,
            Action::WasteNot => 0,
            Action::WasteNot2 => 0,
            Action::GreatStrides => 0,
            Action::Innovation => 0,
            Action::Manipulation => 0,
        };

        if self.buff.waste_not > 0 { d / 2 } else { d }
    }

    pub fn check_action(&self, a:&Action) -> bool {
        if self.cp >= self.get_required_cp(&a) {
            match a {
                Action::ByregotsBlessing => self.buff.inner_quiet >= 1, // ビエルゴはinner_quietが1以上の時に使えます
                Action::PrudentTouch => self.buff.waste_not == 0,       // 倹約加工は倹約が無効の時に使えます
                Action::FocusedTouch => self.buff.observe != 0,         // 注視加工は経過観察の直後のみ使えます(実際はいつでも使えるけど成功率50%は本ツールには適していません)
                _ => true
            }
        }
        else {
            false
        }
    }

    // 効率に対する品質報酬
    // こちらの記事が紹介しているcalculatorの内容を参考にしています。
    // https://jp.finalfantasyxiv.com/lodestone/character/29523439/blog/4641394/
    // 完全一致はしませんが、近似値として使えます。完全一致を求めるならば、データシートを作るほうが良いと思う
    fn quality_reward(&self, setting:&Setting, efficiency : f64) -> u32 {
        let inner_quiet : f64 = From::from(self.buff.inner_quiet);
        let process_accuracy : f64 = From::from(setting.process_accuracy);
        let required_process_accuracy : f64 = From::from(setting.required_process_accuracy);

        let f = if inner_quiet == 0.0 { process_accuracy } else { process_accuracy + process_accuracy * ((inner_quiet-1.0) * 20.0 / 100.0) };
        let q1 = f*35.0/100.0 + 35.0;
        let q2 = q1 * (f + 10000.0) / (required_process_accuracy + 10000.0);
        let q3 = q2 * 60.0 / 100.0;
        let cond_rate = 1.0; // 高品質とか気にしないので常に1.0になります
        let buff_rate = 1.0 + if self.buff.great_strides > 0 { 1.0 } else { 0.0 } + if self.buff.innovation > 0 { 0.5 } else { 0.0 };

        return ( q3 * cond_rate * efficiency * buff_rate ) as u32;
    }

    fn byregots_quality_reward(&self, setting:&Setting) -> u32 {
        self.quality_reward(setting, 1.0 + (self.buff.inner_quiet-1) as f64 * 0.2)
    }

    fn consume_cp(&self, a:&Action) -> State {
        State { cp: self.cp - self.get_required_cp(a), ..*self }
    }

    fn consume_durability(&self, a:&Action) -> State {
        State { durability: self.durability.sub_clip(self.get_required_durability(a)), ..*self }
    }

    fn add_durability(&self, x:u8, setting:&Setting) -> State {
        State { durability: min(self.durability + x, setting.max_durability), ..*self }
    }

    fn set_inner_quiet(&self, x:u8) -> State {
        State { buff: Buff{ inner_quiet: x, ..self.buff }, ..*self }
    }

    fn set_manipulation(&self, x:u8) -> State {
        State { buff: Buff{ manipulation: x, ..self.buff }, ..*self }
    }

    fn set_waste_not(&self, x:u8) -> State {
        State { buff: Buff{ waste_not: x, ..self.buff }, ..*self }
    }

    fn set_great_strides(&self, x:u8) -> State {
        State { buff: Buff{ great_strides: x, ..self.buff }, ..*self }
    }

    fn set_innovation(&self, x:u8) -> State {
        State { buff: Buff{ innovation: x, ..self.buff }, ..*self }
    }

    fn set_basic_touch(&self) -> State {
        State { buff: Buff{ basic_touch: 1, ..self.buff }, ..*self }
    }

    fn set_observe(&self) -> State {
        State { buff: Buff{ observe: 1, ..self.buff }, ..*self }
    }

    fn next_turn(&self, setting:&Setting) -> State {
        State {
            durability: if self.buff.manipulation == 0 || self.durability == 0 { self.durability } else { min(self.durability + 5,setting.max_durability) },
            cp: self.cp,
            buff: self.buff.next_turn(),
        }
    }

    fn run_basic_touch(&self, setting:&Setting) -> (State,u32) {
        (self.consume_cp(&Action::BasicTouch).consume_durability(&Action::BasicTouch).next_turn(setting).set_basic_touch().set_great_strides(0), self.quality_reward(setting, 1.0))
    }

    fn run_standard_touch(&self, setting:&Setting) -> (State,u32) {
        (self.consume_cp(&Action::StandardTouch).consume_durability(&Action::StandardTouch).next_turn(setting).set_great_strides(0), self.quality_reward(setting, 1.25))
    }

    fn run_prudent_touch(&self, setting:&Setting) -> (State,u32) {
        (self.consume_cp(&Action::PrudentTouch).consume_durability(&Action::PrudentTouch).next_turn(setting).set_great_strides(0), self.quality_reward(setting,1.0))
    }

    fn run_focused_touch(&self, setting:&Setting) -> (State,u32) {
        (self.consume_cp(&Action::FocusedTouch).consume_durability(&Action::FocusedTouch).next_turn(setting).set_great_strides(0), self.quality_reward(setting,1.5))
    }

    fn run_preparatory_touch( &self, setting:&Setting ) -> (State,u32) {
        (self.consume_cp(&Action::PreparatoryTouch).consume_durability(&Action::PreparatoryTouch).next_turn(setting).set_great_strides(0), self.quality_reward(setting,2.0))
    }

    fn run_byregots_blessing(&self, setting:&Setting) -> (State,u32) {
        (self.consume_cp(&Action::ByregotsBlessing).consume_durability(&Action::ByregotsBlessing).next_turn(setting).set_inner_quiet(0).set_great_strides(0), self.byregots_quality_reward(setting))
    }

    fn run_masters_mend(&self, setting:&Setting) -> (State,u32) {
        (self.consume_cp(&Action::MastersMend).add_durability(30,setting).next_turn(setting),0)
    }

    fn run_observe(&self, setting:&Setting) -> (State,u32) {
        (self.consume_cp(&Action::Observe).next_turn(setting).set_observe(),0)
    }

    fn run_waste_not(&self, setting:&Setting) -> (State,u32) {
        (self.consume_cp(&Action::WasteNot).next_turn(setting).set_waste_not(4),0)
    }

    fn run_waste_not_2(&self, setting:&Setting) -> (State,u32) {
        (self.consume_cp(&Action::WasteNot2).next_turn(setting).set_waste_not(8),0)
    }

    fn run_great_strides(&self, setting:&Setting) -> (State,u32) {
        (self.consume_cp(&Action::GreatStrides).next_turn(setting).set_great_strides(3),0)
    }

    fn run_innovation(&self, setting:&Setting) -> (State,u32) {
        (self.consume_cp(&Action::Innovation).next_turn(setting).set_innovation(4),0)
    }

    fn run_manipulation(&self, setting:&Setting) -> (State,u32) {
        (self.consume_cp(&Action::Manipulation).set_manipulation(0).next_turn(setting).set_manipulation(8),0)
    }

    pub fn run_action( &self, setting:&Setting, a:&Action ) -> (State,u32) {
        match a {
            Action::CannotAction => panic!("Cannot run action"),
            Action::BasicTouch => self.run_basic_touch(setting),
            Action::StandardTouch => self.run_standard_touch(setting),
            Action::PrudentTouch => self.run_prudent_touch(setting),
            Action::FocusedTouch => self.run_focused_touch(setting),
            Action::PreparatoryTouch => self.run_preparatory_touch(setting),
            Action::ByregotsBlessing => self.run_byregots_blessing(setting),
            Action::MastersMend => self.run_masters_mend(setting),
            Action::Observe => self.run_observe(setting),
            Action::WasteNot => self.run_waste_not(setting),
            Action::WasteNot2 => self.run_waste_not_2(setting),
            Action::GreatStrides => self.run_great_strides(setting),
            Action::Innovation => self.run_innovation(setting),
            Action::Manipulation => self.run_manipulation(setting),
        }
    }
}
