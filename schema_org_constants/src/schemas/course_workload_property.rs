/// <https://schema.org/courseWorkload>
pub const COURSE_WORKLOAD_PROPERTY_IRI_HTTP: &str = "http://schema.org/courseWorkload";
/// <https://schema.org/courseWorkload>
pub const COURSE_WORKLOAD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/courseWorkload";
/// <https://schema.org/courseWorkload>
pub const COURSE_WORKLOAD_PROPERTY_LABEL: &str = "courseWorkload";
pub struct CourseWorkloadPropertyIri;
impl PartialEq<&str> for CourseWorkloadPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COURSE_WORKLOAD_PROPERTY_IRI_HTTP || *other == COURSE_WORKLOAD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CourseWorkloadPropertyIri> for &str {
	fn eq(&self, other: &CourseWorkloadPropertyIri) -> bool {
		*self == COURSE_WORKLOAD_PROPERTY_IRI_HTTP || *self == COURSE_WORKLOAD_PROPERTY_IRI_HTTPS
	}
}
pub struct CourseWorkloadPropertyIriOrLabel;
impl PartialEq<&str> for CourseWorkloadPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CourseWorkloadPropertyIri || *other == COURSE_WORKLOAD_PROPERTY_LABEL
	}
}
impl PartialEq<CourseWorkloadPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CourseWorkloadPropertyIriOrLabel) -> bool {
		*self == CourseWorkloadPropertyIri || *self == COURSE_WORKLOAD_PROPERTY_LABEL
	}
}
