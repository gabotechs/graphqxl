use crate::utils::{escape_non_escaped_quotes, is_last_iter};
use crate::{Synth, SynthContext};
use graphqxl_parser::{ValueBasicData, ValueData};

pub(crate) struct ValueDataSynth(pub(crate) ValueData);

impl Synth for ValueDataSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        // TODO: for now, lets not allow any value to be multiline,
        //  chances that someone wants a multiline value are very low
        let multiline = false;

        match &self.0 {
            ValueData::Basic(value) => match value {
                ValueBasicData::Int(v) => {
                    context.write(&v.to_string());
                    true
                }
                ValueBasicData::Float(v) => {
                    // FIXME: improve this formatting
                    let mut res = v.to_string();
                    if !res.contains('.') {
                        res += ".0";
                    }
                    context.write(&res);
                    true
                }
                ValueBasicData::Boolean(v) => {
                    context.write(&v.to_string());
                    true
                }
                ValueBasicData::String(v) => {
                    context.write(&format!("\"{}\"", escape_non_escaped_quotes(v)));
                    true
                }
            },
            ValueData::List(items) => {
                context.write("[");
                for (is_last, value) in is_last_iter(items.iter()) {
                    if multiline {
                        context.write_line_jump();
                        context.write_indent(context.indent_lvl + 1);
                    } else {
                        context.write(" ");
                    }
                    context.push_indent_level();
                    ValueDataSynth(value.clone()).synth(context);
                    context.pop_indent_level();
                    if !is_last && !multiline {
                        context.write(",");
                    }
                }
                if multiline {
                    context.write_line_jump();
                    context.write_indent(context.indent_lvl);
                } else {
                    context.write(" ");
                }
                context.write("]");
                true
            }
            ValueData::Object(key_values) => {
                context.write("{");
                for (is_last, (key, value)) in is_last_iter(key_values.iter()) {
                    if multiline {
                        context.write_line_jump();
                        context.write_indent(context.indent_lvl + 1);
                    } else {
                        context.write(" ");
                    }
                    context.write(key);
                    context.write(": ");
                    context.push_indent_level();
                    ValueDataSynth(value.clone()).synth(context);
                    context.pop_indent_level();
                    if !is_last && !multiline {
                        context.write(",");
                    }
                }
                if multiline {
                    context.write_line_jump();
                    context.write_indent(context.indent_lvl);
                } else {
                    context.write(" ");
                }
                context.write("}");
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SynthConfig;

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

    #[ignore]
    #[test]
    fn test_list_multiline() {
        let synth = ValueDataSynth(ValueData::int(1).list().push(ValueData::int(2)));
        let mut context = SynthContext::default();
        context.with_config(SynthConfig::default().allow_multiline_values());
        synth.synth(&mut context);
        assert_eq!(
            context.result,
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
        let synth = ValueDataSynth(ValueData::int(1).list().push(ValueData::int(2)));
        let mut context = SynthContext::default();
        context.with_indent_lvl(4);
        context.with_config(SynthConfig::default().allow_multiline_values());
        synth.synth(&mut context);
        assert_eq!(
            context.result,
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

    #[ignore]
    #[test]
    fn test_object_multiline() {
        let synth = ValueDataSynth(
            ValueData::int(1)
                .to_object("a")
                .insert("b", ValueData::int(2)),
        );
        let mut context = SynthContext::default();
        context.with_config(SynthConfig::default().allow_multiline_values());
        synth.synth(&mut context);
        assert_eq!(
            context.result,
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
            ValueData::int(1)
                .to_object("a")
                .insert("b", ValueData::int(2)),
        );
        let mut context = SynthContext::default();
        context.with_indent_lvl(4);
        context.with_config(SynthConfig::default().allow_multiline_values());
        synth.synth(&mut context);
        assert_eq!(
            context.result,
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
    #[ignore]
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
        let mut context = SynthContext::default();
        context.with_config(SynthConfig::default().allow_multiline_values());
        synth.synth(&mut context);
        assert_eq!(
            context.result,
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
