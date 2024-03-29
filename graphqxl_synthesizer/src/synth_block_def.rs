use crate::synth_block_field::BlockFieldSynth;
use crate::synth_description::DescriptionSynth;
use crate::synth_directive::DirectiveSynth;
use crate::synth_identifier::IdentifierSynth;
use crate::synths::{ChainSynth, MultilineListSynth, PairSynth, StringSynth, Synth, SynthContext};
use graphqxl_parser::{BlockDef, BlockDefType, BlockEntry};

pub(crate) struct BlockDefSynth(pub(crate) BlockDef);

impl Synth for BlockDefSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        if self.0.name.id.starts_with(&context.config.private_prefix) {
            return false;
        }

        let mut v: Vec<Box<dyn Synth>> = match self.0.extend {
            true => vec![Box::new(StringSynth::from("extend "))],
            false => vec![],
        };

        let symbol = match self.0.kind {
            BlockDefType::Type => "type",
            BlockDefType::Input => "input",
            BlockDefType::Enum => "enum",
            BlockDefType::Interface => "interface",
        };
        v.push(Box::new(StringSynth::from(symbol)));
        v.push(Box::new(StringSynth::from(" ")));
        v.push(Box::new(IdentifierSynth(self.0.name.clone())));
        if let Some(implements) = &self.0.implements {
            let first = implements.interfaces.get(0).unwrap();
            v.push(Box::new(StringSynth::from(" implements ")));
            v.push(Box::new(IdentifierSynth(first.clone())));
            for i in 1..implements.interfaces.len() {
                let implement = implements.interfaces.get(i).unwrap();
                v.push(Box::new(StringSynth::from(" & ")));
                v.push(Box::new(IdentifierSynth(implement.clone())));
            }
        }
        for directive in self.0.directives.iter() {
            v.push(Box::new(StringSynth::from(" ")));
            v.push(Box::new(DirectiveSynth(directive.clone())));
        }
        let mut inner_synths = Vec::new();
        for entry in self.0.entries.iter() {
            if let BlockEntry::Field(block_field) = entry {
                inner_synths.push(BlockFieldSynth(block_field.clone()));
            }
        }
        if !(inner_synths.is_empty() && self.0.extend) {
            v.push(Box::new(StringSynth::from(" ")));
            v.push(Box::new(MultilineListSynth::no_suffix((
                "{",
                inner_synths,
                "}",
            ))));
        }
        let synth =
            PairSynth::top_level(DescriptionSynth::text(&self.0.description), ChainSynth(v));
        synth.synth(context);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::{Argument, BlockField, Directive, Implements, ValueData};

    fn test_most_simple_block_def_factory() -> BlockDef {
        BlockDef::type_def("MyType").field(BlockField::build("field").string())
    }

    #[test]
    fn test_most_simple_block_def() {
        let synth = BlockDefSynth(test_most_simple_block_def_factory());
        assert_eq!(synth.synth_zero(), "type MyType {\n  field: String\n}")
    }

    fn test_with_args_block_def_factory() -> BlockDef {
        test_most_simple_block_def_factory().field(
            BlockField::build("field2")
                .string()
                .arg(Argument::string("arg1"))
                .arg(Argument::string("arg2"))
                .arg(Argument::string("arg3")),
        )
    }

    #[test]
    fn test_with_args_block_def() {
        let synth = BlockDefSynth(test_with_args_block_def_factory());
        assert_eq!(
            synth.synth_zero(),
            "\
type MyType {
  field: String
  field2(
    arg1: String
    arg2: String
    arg3: String
  ): String
}"
        )
    }

    fn test_with_descriptions_block_def_factory() -> BlockDef {
        test_most_simple_block_def_factory().field(
            BlockField::build("field2")
                .string()
                .description("my description"),
        )
    }

    #[test]
    fn test_with_descriptions_block_def() {
        let synth = BlockDefSynth(test_with_descriptions_block_def_factory());
        assert_eq!(
            synth.synth_zero(),
            "\
type MyType {
  field: String
  \"my description\"
  field2: String
}"
        )
    }

    fn test_with_directive_factory() -> BlockDef {
        test_with_args_block_def_factory()
            .directive(Directive::build("dir1").input("arg", ValueData::int(1)))
            .directive(Directive::build("dir2"))
    }

    #[test]
    fn test_with_directive() {
        let synth = BlockDefSynth(test_with_directive_factory());
        assert_eq!(
            synth.synth_zero(),
            "\
type MyType @dir1(arg: 1) @dir2 {
  field: String
  field2(
    arg1: String
    arg2: String
    arg3: String
  ): String
}"
        );
    }

    #[test]
    fn test_with_implements() {
        let synth = BlockDefSynth(
            test_most_simple_block_def_factory()
                .implements(Implements::from("One").interface("Two")),
        );
        assert_eq!(
            synth.synth_zero(),
            "type MyType implements One & Two {\n  field: String\n}"
        )
    }

    #[test]
    fn test_empty_extension() {
        let synth = BlockDefSynth(BlockDef::type_def("MyType").extend());
        assert_eq!(synth.synth_zero(), "extend type MyType")
    }
}
