use crate::synth_description::DescriptionSynth;
use crate::synths::{ChainSynth, ListSynth, PairSynth, StringSynth};
use crate::{Synth, SynthContext};
use graphqxl_parser::Schema;

pub(crate) struct SchemaSynth(pub(crate) Schema);

impl Synth for SchemaSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let mut to_include = Vec::new();
        if !self.0.query.is_empty() {
            to_include.push(StringSynth(format!("query: {}", self.0.query)))
        }
        if !self.0.mutation.is_empty() {
            to_include.push(StringSynth(format!("mutation: {}", self.0.mutation)))
        }
        if !self.0.subscription.is_empty() {
            to_include.push(StringSynth(format!(
                "subscription: {}",
                self.0.subscription
            )))
        }
        let pair_synth = PairSynth::top_level(
            DescriptionSynth::from(self.0.description.as_str()),
            ChainSynth(vec![
                Box::new(StringSynth("schema ".to_string())),
                Box::new(ListSynth::multiline(("{", to_include, "}"))),
            ]),
        );
        pair_synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_query() {
        let synth = SchemaSynth(Schema::build().query("Query"));
        assert_eq!(synth.synth_zero(), "schema {\n  query: Query\n}")
    }

    #[test]
    fn test_with_query_mutation_subscription() {
        let synth = SchemaSynth(
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
            Schema::build()
                .query("Query")
                .mutation("Mutation")
                .subscription("Subscription"),
        );
        assert_eq!(
            synth.synth_multiline(4),
            "\
schema {
    query: Query
    mutation: Mutation
    subscription: Subscription
}"
        )
    }
}
