use anyhow::{anyhow, Result};
use apollo_compiler::ApolloDiagnostic;
use graphqxl_synthesizer::SourceMapEntry;

struct SourceSpan {
    offset: usize,
    length: usize,
}

fn apollo_diagnostic_source(diagnostic: &ApolloDiagnostic) -> SourceSpan {
    match diagnostic {
        ApolloDiagnostic::MissingIdent(info) => SourceSpan {
            offset: info.definition.offset(),
            length: info.definition.len(),
        },
        ApolloDiagnostic::MissingField(info) => SourceSpan {
            offset: info.current_definition.offset(),
            length: info.current_definition.len(),
        },
        ApolloDiagnostic::UniqueDefinition(info) => SourceSpan {
            offset: info.redefined_definition.offset(),
            length: info.redefined_definition.len(),
        },
        ApolloDiagnostic::SingleRootField(info) => SourceSpan {
            offset: info.subscription.offset(),
            length: info.subscription.len(),
        },
        ApolloDiagnostic::UnsupportedOperation(info) => SourceSpan {
            offset: info.operation.offset(),
            length: info.operation.len(),
        },
        ApolloDiagnostic::SyntaxError(info) => SourceSpan {
            offset: info.span.offset(),
            length: info.span.len(),
        },
        ApolloDiagnostic::UniqueField(info) => SourceSpan {
            offset: info.redefined_field.offset(),
            length: info.redefined_field.len(),
        },
        ApolloDiagnostic::UndefinedDefinition(info) => SourceSpan {
            offset: info.definition.offset(),
            length: info.definition.len(),
        },
        ApolloDiagnostic::UndefinedField(info) => SourceSpan {
            offset: info.definition.offset(),
            length: info.definition.len(),
        },
        ApolloDiagnostic::UniqueArgument(info) => SourceSpan {
            offset: info.redefined_definition.offset(),
            length: info.redefined_definition.len(),
        },
        ApolloDiagnostic::RecursiveDefinition(info) => SourceSpan {
            offset: info.definition.offset(),
            length: info.definition.len(),
        },
        ApolloDiagnostic::TransitiveImplementedInterfaces(info) => SourceSpan {
            offset: info.definition.offset(),
            length: info.definition.len(),
        },
        ApolloDiagnostic::QueryRootOperationType(info) => SourceSpan {
            offset: info.schema.offset(),
            length: info.schema.len(),
        },
        ApolloDiagnostic::BuiltInScalarDefinition(info) => SourceSpan {
            offset: info.scalar.offset(),
            length: info.scalar.len(),
        },
        ApolloDiagnostic::ScalarSpecificationURL(info) => SourceSpan {
            offset: info.scalar.offset(),
            length: info.scalar.len(),
        },
        ApolloDiagnostic::CapitalizedValue(info) => SourceSpan {
            offset: info.value.offset(),
            length: info.value.len(),
        },
        ApolloDiagnostic::UnusedVariable(info) => SourceSpan {
            offset: info.definition.offset(),
            length: info.definition.len(),
        },
        ApolloDiagnostic::OutputType(info) => SourceSpan {
            offset: info.definition.offset(),
            length: info.definition.len(),
        },
        ApolloDiagnostic::ObjectType(info) => SourceSpan {
            offset: info.definition.offset(),
            length: info.definition.len(),
        },
    }
}

pub(crate) fn reverse_diagnostic_map(
    diagnostic: &ApolloDiagnostic,
    source_map: &[SourceMapEntry],
) -> Result<()> {
    let source_span = apollo_diagnostic_source(diagnostic);
    for entry in source_map.iter() {
        let dst_start = source_span.offset;
        let dst_end = source_span.offset + source_span.length;
        let src_start = entry.start;
        let src_end = entry.stop;
        if (src_start <= dst_start && dst_end <= src_end) // if generated span is contained between source span limits
            || (dst_start <= src_start && src_end <= dst_end)
        // if source span is contained between generated span limits
        {
            let err = entry.span.make_error(&diagnostic.report().to_string());
            return Err(anyhow!("{}", err));
        }
    }
    Err(anyhow!("{}", diagnostic))
}
