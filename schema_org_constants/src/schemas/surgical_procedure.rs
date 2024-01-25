/// <https://schema.org/SurgicalProcedure>
pub const SURGICAL_PROCEDURE_IRI_HTTP: &str = "http://schema.org/SurgicalProcedure";
/// <https://schema.org/SurgicalProcedure>
pub const SURGICAL_PROCEDURE_IRI_HTTPS: &str = "https://schema.org/SurgicalProcedure";
/// <https://schema.org/SurgicalProcedure>
pub const SURGICAL_PROCEDURE_LABEL: &str = "SurgicalProcedure";
pub struct SurgicalProcedureIri;
impl PartialEq<&str> for SurgicalProcedureIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SURGICAL_PROCEDURE_IRI_HTTP || *other == SURGICAL_PROCEDURE_IRI_HTTPS
	}
}
impl PartialEq<SurgicalProcedureIri> for &str {
	fn eq(&self, other: &SurgicalProcedureIri) -> bool {
		*self == SURGICAL_PROCEDURE_IRI_HTTP || *self == SURGICAL_PROCEDURE_IRI_HTTPS
	}
}
pub struct SurgicalProcedureIriOrLabel;
impl PartialEq<&str> for SurgicalProcedureIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SurgicalProcedureIri || *other == SURGICAL_PROCEDURE_LABEL
	}
}
impl PartialEq<SurgicalProcedureIriOrLabel> for &str {
	fn eq(&self, other: &SurgicalProcedureIriOrLabel) -> bool {
		*self == SurgicalProcedureIri || *self == SURGICAL_PROCEDURE_LABEL
	}
}
