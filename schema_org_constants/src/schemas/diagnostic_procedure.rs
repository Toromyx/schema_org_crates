/// <https://schema.org/DiagnosticProcedure>
pub const DIAGNOSTIC_PROCEDURE_IRI_HTTP: &str = "http://schema.org/DiagnosticProcedure";
/// <https://schema.org/DiagnosticProcedure>
pub const DIAGNOSTIC_PROCEDURE_IRI_HTTPS: &str = "https://schema.org/DiagnosticProcedure";
/// <https://schema.org/DiagnosticProcedure>
pub const DIAGNOSTIC_PROCEDURE_LABEL: &str = "DiagnosticProcedure";
pub struct DiagnosticProcedureIri;
impl PartialEq<&str> for DiagnosticProcedureIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIAGNOSTIC_PROCEDURE_IRI_HTTP || *other == DIAGNOSTIC_PROCEDURE_IRI_HTTPS
	}
}
impl PartialEq<DiagnosticProcedureIri> for &str {
	fn eq(&self, other: &DiagnosticProcedureIri) -> bool {
		*self == DIAGNOSTIC_PROCEDURE_IRI_HTTP || *self == DIAGNOSTIC_PROCEDURE_IRI_HTTPS
	}
}
pub struct DiagnosticProcedureIriOrLabel;
impl PartialEq<&str> for DiagnosticProcedureIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiagnosticProcedureIri || *other == DIAGNOSTIC_PROCEDURE_LABEL
	}
}
impl PartialEq<DiagnosticProcedureIriOrLabel> for &str {
	fn eq(&self, other: &DiagnosticProcedureIriOrLabel) -> bool {
		*self == DiagnosticProcedureIri || *self == DIAGNOSTIC_PROCEDURE_LABEL
	}
}
