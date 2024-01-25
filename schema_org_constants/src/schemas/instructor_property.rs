/// <https://schema.org/instructor>
pub const INSTRUCTOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/instructor";
/// <https://schema.org/instructor>
pub const INSTRUCTOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/instructor";
/// <https://schema.org/instructor>
pub const INSTRUCTOR_PROPERTY_LABEL: &str = "instructor";
pub struct InstructorPropertyIri;
impl PartialEq<&str> for InstructorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INSTRUCTOR_PROPERTY_IRI_HTTP || *other == INSTRUCTOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InstructorPropertyIri> for &str {
	fn eq(&self, other: &InstructorPropertyIri) -> bool {
		*self == INSTRUCTOR_PROPERTY_IRI_HTTP || *self == INSTRUCTOR_PROPERTY_IRI_HTTPS
	}
}
pub struct InstructorPropertyIriOrLabel;
impl PartialEq<&str> for InstructorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InstructorPropertyIri || *other == INSTRUCTOR_PROPERTY_LABEL
	}
}
impl PartialEq<InstructorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InstructorPropertyIriOrLabel) -> bool {
		*self == InstructorPropertyIri || *self == INSTRUCTOR_PROPERTY_LABEL
	}
}
