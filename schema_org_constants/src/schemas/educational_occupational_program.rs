/// <https://schema.org/EducationalOccupationalProgram>
pub const EDUCATIONAL_OCCUPATIONAL_PROGRAM_IRI_HTTP: &str =
	"http://schema.org/EducationalOccupationalProgram";
/// <https://schema.org/EducationalOccupationalProgram>
pub const EDUCATIONAL_OCCUPATIONAL_PROGRAM_IRI_HTTPS: &str =
	"https://schema.org/EducationalOccupationalProgram";
/// <https://schema.org/EducationalOccupationalProgram>
pub const EDUCATIONAL_OCCUPATIONAL_PROGRAM_LABEL: &str = "EducationalOccupationalProgram";
pub struct EducationalOccupationalProgramIri;
impl PartialEq<&str> for EducationalOccupationalProgramIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDUCATIONAL_OCCUPATIONAL_PROGRAM_IRI_HTTP
			|| *other == EDUCATIONAL_OCCUPATIONAL_PROGRAM_IRI_HTTPS
	}
}
impl PartialEq<EducationalOccupationalProgramIri> for &str {
	fn eq(&self, other: &EducationalOccupationalProgramIri) -> bool {
		*self == EDUCATIONAL_OCCUPATIONAL_PROGRAM_IRI_HTTP
			|| *self == EDUCATIONAL_OCCUPATIONAL_PROGRAM_IRI_HTTPS
	}
}
pub struct EducationalOccupationalProgramIriOrLabel;
impl PartialEq<&str> for EducationalOccupationalProgramIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EducationalOccupationalProgramIri
			|| *other == EDUCATIONAL_OCCUPATIONAL_PROGRAM_LABEL
	}
}
impl PartialEq<EducationalOccupationalProgramIriOrLabel> for &str {
	fn eq(&self, other: &EducationalOccupationalProgramIriOrLabel) -> bool {
		*self == EducationalOccupationalProgramIri
			|| *self == EDUCATIONAL_OCCUPATIONAL_PROGRAM_LABEL
	}
}
