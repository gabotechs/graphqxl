use anyhow::{anyhow, Result};
use apollo_compiler::{diagnostics::DiagnosticData, ApolloDiagnostic};
use graphqxl_synthesizer::SourceMapEntry;

pub(crate) fn is_fatal_diagnostic(diagnostic: &ApolloDiagnostic) -> bool {
    match *diagnostic.data {
        DiagnosticData::SyntaxError { .. } => true,
        DiagnosticData::UniqueDefinition { .. } => true,
        DiagnosticData::UniqueArgument { .. } => true,
        DiagnosticData::UniqueInputValue { .. } => true,
        DiagnosticData::UniqueEnumValue { .. } => true,
        DiagnosticData::RecursiveDirectiveDefinition { .. } => true,
        DiagnosticData::RecursiveInterfaceDefinition { .. } => true,
        DiagnosticData::RecursiveInputObjectDefinition { .. } => true,
        DiagnosticData::RecursiveFragmentDefinition { .. } => true,
        DiagnosticData::DuplicateImplementsInterface { .. } => true,
        DiagnosticData::TransitiveImplementedInterfaces { .. } => true,
        DiagnosticData::OutputType { .. } => true,
        DiagnosticData::InputType { .. } => true,
        DiagnosticData::MissingField { .. } => true,
        DiagnosticData::UndefinedField { .. } => true,
        DiagnosticData::UndefinedArgument { .. } => true,
        DiagnosticData::UndefinedDefinition { .. } => true,

        DiagnosticData::CapitalizedValue { .. } => false,
        DiagnosticData::LimitExceeded { .. } => false,
        DiagnosticData::MissingIdent => false,
        DiagnosticData::ExecutableDefinition { .. } => false,
        DiagnosticData::SingleRootField { .. } => false,
        DiagnosticData::UnsupportedOperation { .. } => false,
        DiagnosticData::UndefinedDirective { .. } => false,
        DiagnosticData::UndefinedVariable { .. } => false,
        DiagnosticData::UndefinedFragment { .. } => false,
        DiagnosticData::UndefinedValue { .. } => false,
        DiagnosticData::WrongTypeExtension { .. } => false,
        DiagnosticData::UniqueField { .. } => false,
        DiagnosticData::RequiredArgument { .. } => false,
        DiagnosticData::ScalarSpecificationURL { .. } => false,
        DiagnosticData::QueryRootOperationType => false,
        DiagnosticData::BuiltInScalarDefinition => false,
        DiagnosticData::VariableInputType { .. } => false,
        DiagnosticData::UnusedVariable { .. } => false,
        DiagnosticData::ObjectType { .. } => false,
        DiagnosticData::UnsupportedDirectiveLocation { .. } => false,
        DiagnosticData::UnsupportedValueType { .. } => false,
        DiagnosticData::IntCoercionError { .. } => false,
        DiagnosticData::UniqueDirective { .. } => false,
        DiagnosticData::IntrospectionField { .. } => false,
        DiagnosticData::DisallowedSubselection { .. } => false,
        DiagnosticData::MissingSubselection { .. } => false,
        DiagnosticData::ConflictingField { .. } => false,
        DiagnosticData::InvalidFragment { .. } => false,
        DiagnosticData::InvalidFragmentTarget { .. } => false,
        DiagnosticData::InvalidFragmentSpread { .. } => false,
        DiagnosticData::UnusedFragment { .. } => false,
        DiagnosticData::DisallowedVariableUsage { .. } => false,
        _ => false,
    }
}

pub(crate) fn reverse_diagnostic_map(
    diagnostic: &ApolloDiagnostic,
    source_map: &[SourceMapEntry],
) -> Result<()> {
    let source_offset = diagnostic.location.offset();
    let source_length = diagnostic.location.node_len();
    for entry in source_map.iter() {
        let dst_start = source_offset;
        let dst_end = source_offset + source_length;
        let src_start = entry.start;
        let src_end = entry.stop;
        if (src_start <= dst_start && dst_end <= src_end) // if generated span is contained between source span limits
            || (dst_start <= src_start && src_end <= dst_end)
        // if source span is contained between generated span limits
        {
            let err = entry.span.make_error(&diagnostic.data.to_string());
            return Err(anyhow!("{err}"));
        }
    }

    Err(anyhow!("{diagnostic}"))
}
