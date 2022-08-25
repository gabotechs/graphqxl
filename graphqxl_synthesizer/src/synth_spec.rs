use crate::synth_block_def::BlockDefSynth;
use crate::synth_directive_def::DirectiveDefSynth;
use crate::synth_scalar::ScalarSynth;
use crate::synth_schema::SchemaSynth;
use crate::synth_union::UnionSynth;
use crate::synths::{Synth, SynthConfig, SynthContext};
use graphqxl_parser::{DefType, Spec};

pub(crate) struct SpecSynth(pub(crate) SynthConfig, pub(crate) Spec);

impl Synth for SpecSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let mut result = "".to_string();
        for def_name in &self.1.order {
            match def_name {
                DefType::Type(name) => {
                    let def = self.1.types.get(name).unwrap().to_owned();
                    result += &BlockDefSynth(self.0, def).synth(context);
                }
                DefType::Input(name) => {
                    let def = self.1.inputs.get(name).unwrap().to_owned();
                    result += &BlockDefSynth(self.0, def).synth(context);
                }
                DefType::Enum(name) => {
                    let def = self.1.enums.get(name).unwrap().to_owned();
                    result += &BlockDefSynth(self.0, def).synth(context);
                }
                DefType::Interface(name) => {
                    let def = self.1.interfaces.get(name).unwrap().to_owned();
                    result += &BlockDefSynth(self.0, def).synth(context);
                }
                DefType::Union(name) => {
                    let def = self.1.unions.get(name).unwrap().to_owned();
                    result += &UnionSynth(self.0, def).synth(context);
                }
                DefType::Scalar(name) => {
                    let def = self.1.scalars.get(name).unwrap().to_owned();
                    result += &ScalarSynth(self.0, def).synth(context);
                }
                DefType::Directive(name) => {
                    let def = self.1.directives.get(name).unwrap().to_owned();
                    result += &DirectiveDefSynth(self.0, def).synth(context);
                }
            }
            result += "\n\n";
        }
        result += &SchemaSynth(self.0, self.1.schema.clone()).synth(context);
        result += "\n";
        result
    }
}
