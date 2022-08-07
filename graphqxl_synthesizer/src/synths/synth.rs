pub(crate) trait Synth {
    fn synth(&self, indent_lvl: usize, multiline: bool) -> String;
    fn synth_zero(&self) -> String {
        self.synth(0, false)
    }
}
