use crate::synths::SynthConfig;
use crate::utils::is_last_iter;
use crate::{Synth, SynthContext};
use graphqxl_parser::{ValueBasicData, ValueData};

pub(crate) struct ValueDataSynth(pub(crate) SynthConfig, pub(crate) ValueData);

impl Synth for ValueDataSynth {
    fn synth(&self, context: &SynthContext) -> String {
        // TODO: for now, lets not allow any value to be multiline,
        //  chances that someone wants a multiline value are very low
        let multiline = false;

        match &self.1 {
            ValueData::Basic(value) => match value {
                ValueBasicData::Int(v) => v.to_string(),
                ValueBasicData::Float(v) => {
                    // FIXME: improve this formatting
                    let mut res = v.to_string();
                    if !res.contains('.') {
                        res += ".0";
                    }
                    res
                }
                ValueBasicData::Boolean(v) => v.to_string(),
                ValueBasicData::String(v) => format!("\"{v}\""),
            },
            ValueData::List(items) => {
                let mut summed = "[".to_string();
                for (is_last, value) in is_last_iter(items.iter()) {
                    if multiline {
                        summed += "\n";
                        summed += " "
                            .repeat(self.0.indent_spaces * (context.indent_lvl + 1))
                            .as_str();
                    } else {
                        summed += " ";
                    }
                    summed += ValueDataSynth(self.0, value.clone())
                        .synth(&context.plus_one_indent_lvl())
                        .as_str();
                    if !is_last && !multiline {
                        summed += ","
                    }
                }
                if multiline {
                    summed += "\n";
                    summed += " "
                        .repeat(self.0.indent_spaces * context.indent_lvl)
                        .as_str();
                } else {
                    summed += " ";
                }
                summed + "]"
            }
            ValueData::Object(key_values) => {
                let mut summed = "{".to_string();
                for (is_last, (key, value)) in is_last_iter(key_values.iter()) {
                    if multiline {
                        summed += "\n";
                        summed += " "
                            .repeat(self.0.indent_spaces * (context.indent_lvl + 1))
                            .as_str();
                    } else {
                        summed += " ";
                    }
                    summed += key;
                    summed += ": ";
                    summed += ValueDataSynth(self.0, value.clone())
                        .synth(&context.plus_one_indent_lvl())
                        .as_str();
                    if !is_last && !multiline {
                        summed += ","
                    }
                }
                if multiline {
                    summed += "\n";
                    summed += " "
                        .repeat(self.0.indent_spaces * context.indent_lvl)
                        .as_str();
                } else {
                    summed += " ";
                }
                summed + "}"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl ValueDataSynth {
        fn default(def: ValueData) -> Self {
            Self(SynthConfig::default(), def)
        }
    }

    #[test]
    fn test_int() {
        let synth = ValueDataSynth::default(ValueData::int(1));
        assert_eq!(synth.synth_zero(), "1")
    }

    #[test]
    fn test_float() {
        let synth = ValueDataSynth::default(ValueData::float(1.0));
        assert_eq!(synth.synth_zero(), "1.0")
    }

    #[test]
    fn test_string() {
        let synth = ValueDataSynth::default(ValueData::string("my data"));
        assert_eq!(synth.synth_zero(), "\"my data\"")
    }

    #[test]
    fn test_boolean() {
        let synth = ValueDataSynth::default(ValueData::boolean(false));
        assert_eq!(synth.synth_zero(), "false")
    }

    #[test]
    fn test_list() {
        let synth = ValueDataSynth::default(ValueData::int(1).list().push(ValueData::int(2)));
        assert_eq!(synth.synth_zero(), "[ 1, 2 ]")
    }

    #[ignore]
    #[test]
    fn test_list_multiline() {
        let synth = ValueDataSynth(
            SynthConfig::default().allow_multiline_values(),
            ValueData::int(1).list().push(ValueData::int(2)),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
[
  1
  2
]"
        )
    }

    #[ignore]
    #[test]
    fn test_list_multiline_indented() {
        let synth = ValueDataSynth(
            SynthConfig::default().allow_multiline_values(),
            ValueData::int(1).list().push(ValueData::int(2)),
        );
        assert_eq!(
            synth.synth(&SynthContext::default().with_indent_lvl(4)),
            "\
[
          1
          2
        ]"
        )
    }

    #[test]
    fn test_object() {
        let synth = ValueDataSynth::default(
            ValueData::int(1)
                .to_object("a")
                .insert("b", ValueData::int(2)),
        );
        assert_eq!(synth.synth_zero(), "{ a: 1, b: 2 }")
    }

    #[ignore]
    #[test]
    fn test_object_multiline() {
        let synth = ValueDataSynth(
            SynthConfig::default().allow_multiline_values(),
            ValueData::int(1)
                .to_object("a")
                .insert("b", ValueData::int(2)),
        );
        assert_eq!(
            synth.synth(&SynthContext::default()),
            "\
{
  a: 1
  b: 2
}"
        );
    }

    #[ignore]
    #[test]
    fn test_object_multiline_indented() {
        let synth = ValueDataSynth(
            SynthConfig::default().allow_multiline_values(),
            ValueData::int(1)
                .to_object("a")
                .insert("b", ValueData::int(2)),
        );
        assert_eq!(
            synth.synth(&SynthContext::default().with_indent_lvl(4)),
            "\
{
          a: 1
          b: 2
        }"
        );
    }

    #[test]
    fn test_deeply_nested() {
        let synth = ValueDataSynth::default(
            ValueData::int(1)
                .to_object("c")
                .list()
                .push(ValueData::string("data").to_object("d"))
                .to_object("a")
                .insert("b", ValueData::int(2)),
        );
        assert_eq!(
            synth.synth_zero(),
            "{ a: [ { c: 1 }, { d: \"data\" } ], b: 2 }"
        )
    }
    #[ignore]
    #[test]
    fn test_deeply_nested_multiline() {
        let synth = ValueDataSynth(
            SynthConfig::default().allow_multiline_values(),
            ValueData::int(1)
                .to_object("c")
                .list()
                .push(ValueData::string("data").to_object("d"))
                .to_object("a")
                .insert("b", ValueData::int(2)),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
{
  a: [
    {
      c: 1
    }
    {
      d: \"data\"
    }
  ]
  b: 2
}"
        )
    }
}
