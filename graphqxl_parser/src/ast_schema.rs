use crate::ast_description::{parse_description_and_continue, DescriptionAndNext};
use crate::ast_identifier::{parse_identifier, Identifier};
use crate::utils::{unknown_rule_error, OwnedSpan};
use crate::Rule;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Schema {
    pub span: OwnedSpan,
    pub description: String,
    // TODO: add directives
    pub query: Identifier,
    pub mutation: Identifier,
    pub subscription: Identifier,
}

impl Schema {
    pub fn build() -> Self {
        Self::default()
    }

    pub fn query(&mut self, query: &str) -> Self {
        self.query = Identifier::from(query);
        self.clone()
    }

    pub fn mutation(&mut self, mutation: &str) -> Self {
        self.mutation = Identifier::from(mutation);
        self.clone()
    }

    pub fn subscription(&mut self, subscription: &str) -> Self {
        self.subscription = Identifier::from(subscription);
        self.clone()
    }

    pub fn description(&mut self, description: &str) -> Self {
        self.description = description.to_string();
        self.clone()
    }
}

enum SchemaKey {
    Query,
    Mutation,
    Subscription,
}

fn parse_schema_key(pair: Pair<Rule>, _file: &str) -> Result<SchemaKey, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::schema_key => {
            return match pair.as_str() {
                "query" => Ok(SchemaKey::Query),
                "mutation" => Ok(SchemaKey::Mutation),
                "subscription" => Ok(SchemaKey::Subscription),
                _ => unreachable!(),
            }
        }
        _unknown => Err(unknown_rule_error(pair, "schema_key")),
    }
}

pub(crate) fn parse_schema(
    pair: Pair<Rule>,
    file: &str,
) -> Result<Schema, pest::error::Error<Rule>> {
    match pair.as_rule() {
        Rule::schema_def => {
            let span = OwnedSpan::from(pair.as_span(), file);
            let mut childs = pair.into_inner();
            let DescriptionAndNext(description, next) =
                parse_description_and_continue(&mut childs, file);
            let mut query = Identifier::from("");
            let mut mutation = Identifier::from("");
            let mut subscription = Identifier::from("");
            for field in next.into_inner() {
                let mut field_parts = field.into_inner();
                let key = parse_schema_key(field_parts.next().unwrap(), file)?;
                let value = parse_identifier(field_parts.next().unwrap(), file)?;
                match key {
                    SchemaKey::Query => query = value,
                    SchemaKey::Mutation => mutation = value,
                    SchemaKey::Subscription => subscription = value,
                }
            }
            Ok(Schema {
                span,
                description,
                query,
                mutation,
                subscription,
            })
        }
        _unknown => Err(unknown_rule_error(pair, "schema")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::parse_full_input;

    fn parse_input(input: &str) -> Result<Schema, pest::error::Error<Rule>> {
        parse_full_input(input, Rule::schema_def, parse_schema)
    }

    #[test]
    fn test_parses_query() {
        assert_eq!(
            parse_input("schema { query: Query }"),
            Ok(Schema::build().query("Query"))
        )
    }

    #[test]
    fn test_parses_query_with_description() {
        assert_eq!(
            parse_input("\"\"\" my \"description \"\"\"schema { query: Query }"),
            Ok(Schema::build()
                .query("Query")
                .description("my \"description"))
        )
    }

    #[test]
    fn test_parses_query_mutation_subscription() {
        assert_eq!(
            parse_input("schema { query: Query mutation: Mutation subscription: Subscription }"),
            Ok(Schema::build()
                .query("Query")
                .mutation("Mutation")
                .subscription("Subscription"))
        )
    }
}
