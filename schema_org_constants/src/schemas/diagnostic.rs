/// <https://schema.org/Diagnostic>
pub const DIAGNOSTIC_IRI_HTTP: &str = "http://schema.org/Diagnostic";
/// <https://schema.org/Diagnostic>
pub const DIAGNOSTIC_IRI_HTTPS: &str = "https://schema.org/Diagnostic";
/// <https://schema.org/Diagnostic>
pub const DIAGNOSTIC_LABEL: &str = "Diagnostic";
pub struct DiagnosticIri;
impl PartialEq<&str> for DiagnosticIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIAGNOSTIC_IRI_HTTP || *other == DIAGNOSTIC_IRI_HTTPS
	}
}
impl PartialEq<DiagnosticIri> for &str {
	fn eq(&self, other: &DiagnosticIri) -> bool {
		*self == DIAGNOSTIC_IRI_HTTP || *self == DIAGNOSTIC_IRI_HTTPS
	}
}
pub struct DiagnosticIriOrLabel;
impl PartialEq<&str> for DiagnosticIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiagnosticIri || *other == DIAGNOSTIC_LABEL
	}
}
impl PartialEq<DiagnosticIriOrLabel> for &str {
	fn eq(&self, other: &DiagnosticIriOrLabel) -> bool {
		*self == DiagnosticIri || *self == DIAGNOSTIC_LABEL
	}
}
