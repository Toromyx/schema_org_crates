/// <https://schema.org/CourseInstance>
pub const COURSE_INSTANCE_IRI_HTTP: &str = "http://schema.org/CourseInstance";
/// <https://schema.org/CourseInstance>
pub const COURSE_INSTANCE_IRI_HTTPS: &str = "https://schema.org/CourseInstance";
/// <https://schema.org/CourseInstance>
pub const COURSE_INSTANCE_LABEL: &str = "CourseInstance";
pub struct CourseInstanceIri;
impl PartialEq<&str> for CourseInstanceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COURSE_INSTANCE_IRI_HTTP || *other == COURSE_INSTANCE_IRI_HTTPS
	}
}
impl PartialEq<CourseInstanceIri> for &str {
	fn eq(&self, other: &CourseInstanceIri) -> bool {
		*self == COURSE_INSTANCE_IRI_HTTP || *self == COURSE_INSTANCE_IRI_HTTPS
	}
}
pub struct CourseInstanceIriOrLabel;
impl PartialEq<&str> for CourseInstanceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CourseInstanceIri || *other == COURSE_INSTANCE_LABEL
	}
}
impl PartialEq<CourseInstanceIriOrLabel> for &str {
	fn eq(&self, other: &CourseInstanceIriOrLabel) -> bool {
		*self == CourseInstanceIri || *self == COURSE_INSTANCE_LABEL
	}
}
