/// <https://schema.org/GolfCourse>
pub const GOLF_COURSE_IRI_HTTP: &str = "http://schema.org/GolfCourse";
/// <https://schema.org/GolfCourse>
pub const GOLF_COURSE_IRI_HTTPS: &str = "https://schema.org/GolfCourse";
/// <https://schema.org/GolfCourse>
pub const GOLF_COURSE_LABEL: &str = "GolfCourse";
pub struct GolfCourseIri;
impl PartialEq<&str> for GolfCourseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GOLF_COURSE_IRI_HTTP || *other == GOLF_COURSE_IRI_HTTPS
	}
}
impl PartialEq<GolfCourseIri> for &str {
	fn eq(&self, other: &GolfCourseIri) -> bool {
		*self == GOLF_COURSE_IRI_HTTP || *self == GOLF_COURSE_IRI_HTTPS
	}
}
pub struct GolfCourseIriOrLabel;
impl PartialEq<&str> for GolfCourseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GolfCourseIri || *other == GOLF_COURSE_LABEL
	}
}
impl PartialEq<GolfCourseIriOrLabel> for &str {
	fn eq(&self, other: &GolfCourseIriOrLabel) -> bool {
		*self == GolfCourseIri || *self == GOLF_COURSE_LABEL
	}
}
