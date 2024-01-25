/// <https://schema.org/coursePrerequisites>
pub const COURSE_PREREQUISITES_PROPERTY_IRI_HTTP: &str = "http://schema.org/coursePrerequisites";
/// <https://schema.org/coursePrerequisites>
pub const COURSE_PREREQUISITES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/coursePrerequisites";
/// <https://schema.org/coursePrerequisites>
pub const COURSE_PREREQUISITES_PROPERTY_LABEL: &str = "coursePrerequisites";
pub struct CoursePrerequisitesPropertyIri;
impl PartialEq<&str> for CoursePrerequisitesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COURSE_PREREQUISITES_PROPERTY_IRI_HTTP
			|| *other == COURSE_PREREQUISITES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CoursePrerequisitesPropertyIri> for &str {
	fn eq(&self, other: &CoursePrerequisitesPropertyIri) -> bool {
		*self == COURSE_PREREQUISITES_PROPERTY_IRI_HTTP
			|| *self == COURSE_PREREQUISITES_PROPERTY_IRI_HTTPS
	}
}
pub struct CoursePrerequisitesPropertyIriOrLabel;
impl PartialEq<&str> for CoursePrerequisitesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CoursePrerequisitesPropertyIri || *other == COURSE_PREREQUISITES_PROPERTY_LABEL
	}
}
impl PartialEq<CoursePrerequisitesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CoursePrerequisitesPropertyIriOrLabel) -> bool {
		*self == CoursePrerequisitesPropertyIri || *self == COURSE_PREREQUISITES_PROPERTY_LABEL
	}
}
