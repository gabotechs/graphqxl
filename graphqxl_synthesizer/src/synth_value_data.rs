use crate::utils::is_last_iter;
use crate::{Synth, SynthContext};
use graphqxl_parser::{ValueBasicData, ValueData};

pub(crate) struct ValueDataSynth(pub(crate) ValueData);

impl Synth for ValueDataSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let context = if !context.allow_multiline_values {
            context.no_multiline()
        } else {
            *context
        };
        match &self.0 {
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
                    if context.multiline {
                        summed += "\n";
                        summed += " "
                            .repeat(context.indent_spaces * (context.indent_lvl + 1))
                            .as_str();
                    } else {
                        summed += " ";
                    }
                    summed += ValueDataSynth(value.clone())
                        .synth(&context.plus_one_indent_lvl())
                        .as_str();
                    if !is_last && !context.multiline {
                        summed += ","
                    }
                }
                if context.multiline {
                    summed += "\n";
                    summed += " "
                        .repeat(context.indent_spaces * context.indent_lvl)
                        .as_str();
                } else {
                    summed += " ";
                }
                summed + "]"
            }
            ValueData::Object(key_values) => {
                let mut summed = "{".to_string();
                for (is_last, (key, value)) in is_last_iter(key_values.iter()) {
                    if context.multiline {
                        summed += "\n";
                        summed += " "
                            .repeat(context.indent_spaces * (context.indent_lvl + 1))
                            .as_str();
                    } else {
                        summed += " ";
                    }
                    summed += key;
                    summed += ": ";
                    summed += ValueDataSynth(value.clone())
                        .synth(&context.plus_one_indent_lvl())
                        .as_str();
                    if !is_last && !context.multiline {
                        summed += ","
                    }
                }
                if context.multiline {
                    summed += "\n";
                    summed += " "
                        .repeat(context.indent_spaces * context.indent_lvl)
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

    #[test]
    fn test_int() {
        let synth = ValueDataSynth(ValueData::int(1));
        assert_eq!(synth.synth_zero(), "1")
    }

    #[test]
    fn test_float() {
        let synth = ValueDataSynth(ValueData::float(1.0));
        assert_eq!(synth.synth_zero(), "1.0")
    }

    #[test]
    fn test_string() {
        let synth = ValueDataSynth(ValueData::string("my data"));
        assert_eq!(synth.synth_zero(), "\"my data\"")
    }

    #[test]
    fn test_boolean() {
        let synth = ValueDataSynth(ValueData::boolean(false));
        assert_eq!(synth.synth_zero(), "false")
    }

    #[test]
    fn test_list() {
        let synth = ValueDataSynth(ValueData::int(1).list().push(ValueData::int(2)));
        assert_eq!(synth.synth_zero(), "[ 1, 2 ]")
    }

    #[test]
    fn test_list_multiline() {
        let synth = ValueDataSynth(ValueData::int(1).list().push(ValueData::int(2)));
        assert_eq!(
            synth.synth(&SynthContext::default().multiline().allow_multiline_values()),
            "\
[
  1
  2
]"
        )
    }

    #[test]
    fn test_list_multiline_indented() {
        let synth = ValueDataSynth(ValueData::int(1).list().push(ValueData::int(2)));
        assert_eq!(
            synth.synth(
                &SynthContext::default()
                    .multiline()
                    .allow_multiline_values()
                    .with_indent_lvl(4)
            ),
            "\
[
          1
          2
        ]"
        )
    }

    #[test]
    fn test_object() {
        let synth = ValueDataSynth(
            ValueData::int(1)
                .to_object("a")
                .insert("b", ValueData::int(2)),
        );
        assert_eq!(synth.synth_zero(), "{ a: 1, b: 2 }")
    }

    #[test]
    fn test_object_multiline() {
        let synth = ValueDataSynth(
            ValueData::int(1)
                .to_object("a")
                .insert("b", ValueData::int(2)),
        );
        assert_eq!(
            synth.synth(&SynthContext::default().multiline().allow_multiline_values()),
            "\
{
  a: 1
  b: 2
}"
        );
    }

    #[test]
    fn test_object_multiline_indented() {
        let synth = ValueDataSynth(
            ValueData::int(1)
                .to_object("a")
                .insert("b", ValueData::int(2)),
        );
        assert_eq!(
            synth.synth(
                &SynthContext::default()
                    .multiline()
                    .allow_multiline_values()
                    .with_indent_lvl(4)
            ),
            "\
{
          a: 1
          b: 2
        }"
        );
    }

    #[test]
    fn test_deeply_nested() {
        let synth = ValueDataSynth(
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
    #[test]
    fn test_deeply_nested_multiline() {
        let synth = ValueDataSynth(
            ValueData::int(1)
                .to_object("c")
                .list()
                .push(ValueData::string("data").to_object("d"))
                .to_object("a")
                .insert("b", ValueData::int(2)),
        );
        assert_eq!(
            synth.synth(&SynthContext::default().multiline().allow_multiline_values()),
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
