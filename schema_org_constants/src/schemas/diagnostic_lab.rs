/// <https://schema.org/DiagnosticLab>
pub const DIAGNOSTIC_LAB_IRI_HTTP: &str = "http://schema.org/DiagnosticLab";
/// <https://schema.org/DiagnosticLab>
pub const DIAGNOSTIC_LAB_IRI_HTTPS: &str = "https://schema.org/DiagnosticLab";
/// <https://schema.org/DiagnosticLab>
pub const DIAGNOSTIC_LAB_LABEL: &str = "DiagnosticLab";
pub struct DiagnosticLabIri;
impl PartialEq<&str> for DiagnosticLabIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIAGNOSTIC_LAB_IRI_HTTP || *other == DIAGNOSTIC_LAB_IRI_HTTPS
	}
}
impl PartialEq<DiagnosticLabIri> for &str {
	fn eq(&self, other: &DiagnosticLabIri) -> bool {
		*self == DIAGNOSTIC_LAB_IRI_HTTP || *self == DIAGNOSTIC_LAB_IRI_HTTPS
	}
}
pub struct DiagnosticLabIriOrLabel;
impl PartialEq<&str> for DiagnosticLabIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiagnosticLabIri || *other == DIAGNOSTIC_LAB_LABEL
	}
}
impl PartialEq<DiagnosticLabIriOrLabel> for &str {
	fn eq(&self, other: &DiagnosticLabIriOrLabel) -> bool {
		*self == DiagnosticLabIri || *self == DIAGNOSTIC_LAB_LABEL
	}
}
