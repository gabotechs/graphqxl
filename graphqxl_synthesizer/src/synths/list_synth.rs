use crate::synths::synth::Synth;
use crate::utils::is_last_iter;

pub(crate) struct ListSynth<T: Synth> {
    pub(crate) indent: usize,
    pub(crate) items: Vec<T>,
    pub(crate) sep: String,
    pub(crate) wrapper: (String, String),
}

impl<T: Synth> Synth for ListSynth<T> {
    fn synth(&self, indent_lvl: usize, multiline: bool) -> String {
        let mut result = self.wrapper.0.clone();
        for (is_last, item) in is_last_iter(self.items.iter()) {
            if multiline {
                result += "\n";
                result += &" ".repeat((indent_lvl + 1) * self.indent);
                result += &item.synth(indent_lvl, multiline);
                if !is_last {
                    result += &self.sep;
                }
            } else {
                result += &item.synth(indent_lvl, multiline);
                if !is_last {
                    result += &self.sep
                }
            }
        }
        if multiline {
            result += "\n";
            result += &" ".repeat((indent_lvl) * self.indent);
        }
        result + &self.wrapper.1
    }
}
