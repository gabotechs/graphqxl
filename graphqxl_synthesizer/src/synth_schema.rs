use crate::synth_description::DescriptionSynth;
use crate::synths::{ChainSynth, MultilineListSynth, PairSynth, StringSynth, SynthConfig};
use crate::{Synth, SynthContext};
use graphqxl_parser::Schema;

pub(crate) struct SchemaSynth(pub(crate) SynthConfig, pub(crate) Schema);

impl Synth for SchemaSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let mut to_include = Vec::new();
        if !self.1.query.is_empty() {
            to_include.push(StringSynth(format!("query: {}", self.1.query)))
        }
        if !self.1.mutation.is_empty() {
            to_include.push(StringSynth(format!("mutation: {}", self.1.mutation)))
        }
        if !self.1.subscription.is_empty() {
            to_include.push(StringSynth(format!(
                "subscription: {}",
                self.1.subscription
            )))
        }
        let pair_synth = PairSynth::top_level(
            &self.0,
            DescriptionSynth::text(&self.0, self.1.description.as_str()),
            ChainSynth(vec![
                Box::new(StringSynth("schema ".to_string())),
                Box::new(MultilineListSynth::no_suffix(
                    &self.0,
                    ("{", to_include, "}"),
                )),
            ]),
        );
        pair_synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl SchemaSynth {
        fn default(def: Schema) -> Self {
            Self(SynthConfig::default(), def)
        }
    }

    #[test]
    fn test_with_query() {
        let synth = SchemaSynth::default(Schema::build().query("Query"));
        assert_eq!(synth.synth_zero(), "schema {\n  query: Query\n}")
    }

    #[test]
    fn test_with_query_mutation_subscription() {
        let synth = SchemaSynth::default(
            Schema::build()
                .query("Query")
                .mutation("Mutation")
                .subscription("Subscription"),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
schema {
  query: Query
  mutation: Mutation
  subscription: Subscription
}"
        )
    }

    #[test]
    fn test_with_query_mutation_subscription_indented() {
        let synth = SchemaSynth(
            SynthConfig::default().indent_spaces(4),
            Schema::build()
                .query("Query")
                .mutation("Mutation")
                .subscription("Subscription"),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
schema {
    query: Query
    mutation: Mutation
    subscription: Subscription
}"
        )
    }
}
