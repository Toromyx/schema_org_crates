/// <https://schema.org/procedure>
pub const PROCEDURE_PROPERTY_IRI_HTTP: &str = "http://schema.org/procedure";
/// <https://schema.org/procedure>
pub const PROCEDURE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/procedure";
/// <https://schema.org/procedure>
pub const PROCEDURE_PROPERTY_LABEL: &str = "procedure";
pub struct ProcedurePropertyIri;
impl PartialEq<&str> for ProcedurePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROCEDURE_PROPERTY_IRI_HTTP || *other == PROCEDURE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProcedurePropertyIri> for &str {
	fn eq(&self, other: &ProcedurePropertyIri) -> bool {
		*self == PROCEDURE_PROPERTY_IRI_HTTP || *self == PROCEDURE_PROPERTY_IRI_HTTPS
	}
}
pub struct ProcedurePropertyIriOrLabel;
impl PartialEq<&str> for ProcedurePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProcedurePropertyIri || *other == PROCEDURE_PROPERTY_LABEL
	}
}
impl PartialEq<ProcedurePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProcedurePropertyIriOrLabel) -> bool {
		*self == ProcedurePropertyIri || *self == PROCEDURE_PROPERTY_LABEL
	}
}
