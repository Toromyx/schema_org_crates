/// <https://schema.org/programType>
pub const PROGRAM_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/programType";
/// <https://schema.org/programType>
pub const PROGRAM_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/programType";
/// <https://schema.org/programType>
pub const PROGRAM_TYPE_PROPERTY_LABEL: &str = "programType";
pub struct ProgramTypePropertyIri;
impl PartialEq<&str> for ProgramTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROGRAM_TYPE_PROPERTY_IRI_HTTP || *other == PROGRAM_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProgramTypePropertyIri> for &str {
	fn eq(&self, other: &ProgramTypePropertyIri) -> bool {
		*self == PROGRAM_TYPE_PROPERTY_IRI_HTTP || *self == PROGRAM_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct ProgramTypePropertyIriOrLabel;
impl PartialEq<&str> for ProgramTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProgramTypePropertyIri || *other == PROGRAM_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<ProgramTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProgramTypePropertyIriOrLabel) -> bool {
		*self == ProgramTypePropertyIri || *self == PROGRAM_TYPE_PROPERTY_LABEL
	}
}
