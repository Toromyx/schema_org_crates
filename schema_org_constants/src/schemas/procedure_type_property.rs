/// <https://schema.org/procedureType>
pub const PROCEDURE_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/procedureType";
/// <https://schema.org/procedureType>
pub const PROCEDURE_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/procedureType";
/// <https://schema.org/procedureType>
pub const PROCEDURE_TYPE_PROPERTY_LABEL: &str = "procedureType";
pub struct ProcedureTypePropertyIri;
impl PartialEq<&str> for ProcedureTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROCEDURE_TYPE_PROPERTY_IRI_HTTP || *other == PROCEDURE_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProcedureTypePropertyIri> for &str {
	fn eq(&self, other: &ProcedureTypePropertyIri) -> bool {
		*self == PROCEDURE_TYPE_PROPERTY_IRI_HTTP || *self == PROCEDURE_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct ProcedureTypePropertyIriOrLabel;
impl PartialEq<&str> for ProcedureTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProcedureTypePropertyIri || *other == PROCEDURE_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<ProcedureTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProcedureTypePropertyIriOrLabel) -> bool {
		*self == ProcedureTypePropertyIri || *self == PROCEDURE_TYPE_PROPERTY_LABEL
	}
}
