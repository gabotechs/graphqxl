use crate::synth_description::DescriptionSynth;
use crate::synth_directive::DirectiveSynth;
use crate::synths::{ChainSynth, MultilineListSynth, PairSynth, StringSynth};
use crate::{Synth, SynthContext};
use graphqxl_parser::Schema;

pub(crate) struct SchemaSynth(pub(crate) Schema);

impl Synth for SchemaSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        let mut to_include = Vec::new();
        if !self.0.query.id.is_empty() {
            to_include.push(StringSynth(format!("query: {}", self.0.query.id)))
        }
        if !self.0.mutation.id.is_empty() {
            to_include.push(StringSynth(format!("mutation: {}", self.0.mutation.id)))
        }
        if !self.0.subscription.id.is_empty() {
            to_include.push(StringSynth(format!(
                "subscription: {}",
                self.0.subscription.id
            )))
        }
        if to_include.is_empty() && !self.0.extend {
            return false;
        }

        let mut v: Vec<Box<dyn Synth>> = match self.0.extend {
            true => vec![Box::new(StringSynth::from("extend "))],
            false => vec![],
        };

        v.push(Box::new(StringSynth::from("schema")));
        for directive in self.0.directives.iter() {
            v.push(Box::new(StringSynth::from(" ")));
            v.push(Box::new(DirectiveSynth(directive.clone())));
        }
        if !(self.0.extend && to_include.is_empty()) {
            v.push(Box::new(StringSynth::from(" ")));
            v.push(Box::new(MultilineListSynth::no_suffix((
                "{", to_include, "}",
            ))));
        }
        let pair_synth =
            PairSynth::top_level(DescriptionSynth::text(&self.0.description), ChainSynth(v));
        pair_synth.synth(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SynthConfig;
    use graphqxl_parser::Directive;

    #[test]
    fn test_with_query() {
        let synth = SchemaSynth(Schema::build().query("Query"));
        assert_eq!(synth.synth_zero(), "schema {\n  query: Query\n}")
    }

    #[test]
    fn test_empty_with_extension() {
        let synth = SchemaSynth(Schema::build().extend());
        assert_eq!(synth.synth_zero(), "extend schema")
    }

    #[test]
    fn test_with_query_and_directives() {
        let synth = SchemaSynth(
            Schema::build()
                .query("Query")
                .directive(Directive::build("dir1"))
                .directive(Directive::build("dir2")),
        );
        assert_eq!(
            synth.synth_zero(),
            "schema @dir1 @dir2 {\n  query: Query\n}"
        )
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
        let mut context = SynthContext::default();
        context.with_config(SynthConfig::default().indent_spaces(4));
        synth.synth(&mut context);
        assert_eq!(
            context.result,
            "\
schema {
    query: Query
    mutation: Mutation
    subscription: Subscription
}"
        )
    }
}
