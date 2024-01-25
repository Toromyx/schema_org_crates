/// <https://schema.org/hasCourse>
pub const HAS_COURSE_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasCourse";
/// <https://schema.org/hasCourse>
pub const HAS_COURSE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasCourse";
/// <https://schema.org/hasCourse>
pub const HAS_COURSE_PROPERTY_LABEL: &str = "hasCourse";
pub struct HasCoursePropertyIri;
impl PartialEq<&str> for HasCoursePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_COURSE_PROPERTY_IRI_HTTP || *other == HAS_COURSE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasCoursePropertyIri> for &str {
	fn eq(&self, other: &HasCoursePropertyIri) -> bool {
		*self == HAS_COURSE_PROPERTY_IRI_HTTP || *self == HAS_COURSE_PROPERTY_IRI_HTTPS
	}
}
pub struct HasCoursePropertyIriOrLabel;
impl PartialEq<&str> for HasCoursePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasCoursePropertyIri || *other == HAS_COURSE_PROPERTY_LABEL
	}
}
impl PartialEq<HasCoursePropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasCoursePropertyIriOrLabel) -> bool {
		*self == HasCoursePropertyIri || *self == HAS_COURSE_PROPERTY_LABEL
	}
}
