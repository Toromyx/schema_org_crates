/// <https://schema.org/programName>
pub const PROGRAM_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/programName";
/// <https://schema.org/programName>
pub const PROGRAM_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/programName";
/// <https://schema.org/programName>
pub const PROGRAM_NAME_PROPERTY_LABEL: &str = "programName";
pub struct ProgramNamePropertyIri;
impl PartialEq<&str> for ProgramNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROGRAM_NAME_PROPERTY_IRI_HTTP || *other == PROGRAM_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProgramNamePropertyIri> for &str {
	fn eq(&self, other: &ProgramNamePropertyIri) -> bool {
		*self == PROGRAM_NAME_PROPERTY_IRI_HTTP || *self == PROGRAM_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct ProgramNamePropertyIriOrLabel;
impl PartialEq<&str> for ProgramNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProgramNamePropertyIri || *other == PROGRAM_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<ProgramNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProgramNamePropertyIriOrLabel) -> bool {
		*self == ProgramNamePropertyIri || *self == PROGRAM_NAME_PROPERTY_LABEL
	}
}
