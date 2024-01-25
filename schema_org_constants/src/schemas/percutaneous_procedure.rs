/// <https://schema.org/PercutaneousProcedure>
pub const PERCUTANEOUS_PROCEDURE_IRI_HTTP: &str = "http://schema.org/PercutaneousProcedure";
/// <https://schema.org/PercutaneousProcedure>
pub const PERCUTANEOUS_PROCEDURE_IRI_HTTPS: &str = "https://schema.org/PercutaneousProcedure";
/// <https://schema.org/PercutaneousProcedure>
pub const PERCUTANEOUS_PROCEDURE_LABEL: &str = "PercutaneousProcedure";
pub struct PercutaneousProcedureIri;
impl PartialEq<&str> for PercutaneousProcedureIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERCUTANEOUS_PROCEDURE_IRI_HTTP || *other == PERCUTANEOUS_PROCEDURE_IRI_HTTPS
	}
}
impl PartialEq<PercutaneousProcedureIri> for &str {
	fn eq(&self, other: &PercutaneousProcedureIri) -> bool {
		*self == PERCUTANEOUS_PROCEDURE_IRI_HTTP || *self == PERCUTANEOUS_PROCEDURE_IRI_HTTPS
	}
}
pub struct PercutaneousProcedureIriOrLabel;
impl PartialEq<&str> for PercutaneousProcedureIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PercutaneousProcedureIri || *other == PERCUTANEOUS_PROCEDURE_LABEL
	}
}
impl PartialEq<PercutaneousProcedureIriOrLabel> for &str {
	fn eq(&self, other: &PercutaneousProcedureIriOrLabel) -> bool {
		*self == PercutaneousProcedureIri || *self == PERCUTANEOUS_PROCEDURE_LABEL
	}
}
