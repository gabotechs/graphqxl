use crate::synth_block_def::BlockDefSynth;
use crate::synth_directive_def::DirectiveDefSynth;
use crate::synth_scalar::ScalarSynth;
use crate::synth_union::UnionSynth;
use crate::synths::{Synth, SynthContext};
use graphqxl_parser::{DefType, Spec};

pub(crate) struct SpecSynth(pub(crate) Spec);

impl Synth for SpecSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let mut result = "".to_string();
        for def_name in &self.0.order {
            match def_name {
                DefType::Type(name) => {
                    let def = self.0.types.get(name).unwrap().to_owned();
                    result += &BlockDefSynth(def).synth(context);
                }
                DefType::Input(name) => {
                    let def = self.0.inputs.get(name).unwrap().to_owned();
                    result += &BlockDefSynth(def).synth(context);
                }
                DefType::Enum(name) => {
                    let def = self.0.enums.get(name).unwrap().to_owned();
                    result += &BlockDefSynth(def).synth(context);
                }
                DefType::Interface(name) => {
                    let def = self.0.interfaces.get(name).unwrap().to_owned();
                    result += &BlockDefSynth(def).synth(context);
                }
                DefType::Union(name) => {
                    let def = self.0.unions.get(name).unwrap().to_owned();
                    result += &UnionSynth(def).synth(context);
                }
                DefType::Scalar(name) => {
                    let def = self.0.scalars.get(name).unwrap().to_owned();
                    result += &ScalarSynth(def).synth(context);
                }
                DefType::Directive(name) => {
                    let def = self.0.directives.get(name).unwrap().to_owned();
                    result += &DirectiveDefSynth(def).synth(context);
                }
            }
            result += "\n\n";
        }
        result
    }
}
