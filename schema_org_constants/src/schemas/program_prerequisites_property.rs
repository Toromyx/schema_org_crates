/// <https://schema.org/programPrerequisites>
pub const PROGRAM_PREREQUISITES_PROPERTY_IRI_HTTP: &str = "http://schema.org/programPrerequisites";
/// <https://schema.org/programPrerequisites>
pub const PROGRAM_PREREQUISITES_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/programPrerequisites";
/// <https://schema.org/programPrerequisites>
pub const PROGRAM_PREREQUISITES_PROPERTY_LABEL: &str = "programPrerequisites";
pub struct ProgramPrerequisitesPropertyIri;
impl PartialEq<&str> for ProgramPrerequisitesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROGRAM_PREREQUISITES_PROPERTY_IRI_HTTP
			|| *other == PROGRAM_PREREQUISITES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProgramPrerequisitesPropertyIri> for &str {
	fn eq(&self, other: &ProgramPrerequisitesPropertyIri) -> bool {
		*self == PROGRAM_PREREQUISITES_PROPERTY_IRI_HTTP
			|| *self == PROGRAM_PREREQUISITES_PROPERTY_IRI_HTTPS
	}
}
pub struct ProgramPrerequisitesPropertyIriOrLabel;
impl PartialEq<&str> for ProgramPrerequisitesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProgramPrerequisitesPropertyIri || *other == PROGRAM_PREREQUISITES_PROPERTY_LABEL
	}
}
impl PartialEq<ProgramPrerequisitesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProgramPrerequisitesPropertyIriOrLabel) -> bool {
		*self == ProgramPrerequisitesPropertyIri || *self == PROGRAM_PREREQUISITES_PROPERTY_LABEL
	}
}
