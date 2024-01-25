/// <https://schema.org/WorkBasedProgram>
pub const WORK_BASED_PROGRAM_IRI_HTTP: &str = "http://schema.org/WorkBasedProgram";
/// <https://schema.org/WorkBasedProgram>
pub const WORK_BASED_PROGRAM_IRI_HTTPS: &str = "https://schema.org/WorkBasedProgram";
/// <https://schema.org/WorkBasedProgram>
pub const WORK_BASED_PROGRAM_LABEL: &str = "WorkBasedProgram";
pub struct WorkBasedProgramIri;
impl PartialEq<&str> for WorkBasedProgramIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WORK_BASED_PROGRAM_IRI_HTTP || *other == WORK_BASED_PROGRAM_IRI_HTTPS
	}
}
impl PartialEq<WorkBasedProgramIri> for &str {
	fn eq(&self, other: &WorkBasedProgramIri) -> bool {
		*self == WORK_BASED_PROGRAM_IRI_HTTP || *self == WORK_BASED_PROGRAM_IRI_HTTPS
	}
}
pub struct WorkBasedProgramIriOrLabel;
impl PartialEq<&str> for WorkBasedProgramIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WorkBasedProgramIri || *other == WORK_BASED_PROGRAM_LABEL
	}
}
impl PartialEq<WorkBasedProgramIriOrLabel> for &str {
	fn eq(&self, other: &WorkBasedProgramIriOrLabel) -> bool {
		*self == WorkBasedProgramIri || *self == WORK_BASED_PROGRAM_LABEL
	}
}
