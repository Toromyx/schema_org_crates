/// <https://schema.org/course>
#[deprecated = "This schema is superseded by <https://schema.org/exerciseCourse>."]
pub const COURSE_PROPERTY_IRI_HTTP: &str = "http://schema.org/course";
/// <https://schema.org/course>
#[deprecated = "This schema is superseded by <https://schema.org/exerciseCourse>."]
pub const COURSE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/course";
/// <https://schema.org/course>
#[deprecated = "This schema is superseded by <https://schema.org/exerciseCourse>."]
pub const COURSE_PROPERTY_LABEL: &str = "course";
pub struct CoursePropertyIri;
impl PartialEq<&str> for CoursePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COURSE_PROPERTY_IRI_HTTP || *other == COURSE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CoursePropertyIri> for &str {
	fn eq(&self, other: &CoursePropertyIri) -> bool {
		*self == COURSE_PROPERTY_IRI_HTTP || *self == COURSE_PROPERTY_IRI_HTTPS
	}
}
pub struct CoursePropertyIriOrLabel;
impl PartialEq<&str> for CoursePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CoursePropertyIri || *other == COURSE_PROPERTY_LABEL
	}
}
impl PartialEq<CoursePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CoursePropertyIriOrLabel) -> bool {
		*self == CoursePropertyIri || *self == COURSE_PROPERTY_LABEL
	}
}
