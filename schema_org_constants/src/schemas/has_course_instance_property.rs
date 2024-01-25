/// <https://schema.org/hasCourseInstance>
pub const HAS_COURSE_INSTANCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasCourseInstance";
/// <https://schema.org/hasCourseInstance>
pub const HAS_COURSE_INSTANCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasCourseInstance";
/// <https://schema.org/hasCourseInstance>
pub const HAS_COURSE_INSTANCE_PROPERTY_LABEL: &str = "hasCourseInstance";
pub struct HasCourseInstancePropertyIri;
impl PartialEq<&str> for HasCourseInstancePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_COURSE_INSTANCE_PROPERTY_IRI_HTTP
			|| *other == HAS_COURSE_INSTANCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasCourseInstancePropertyIri> for &str {
	fn eq(&self, other: &HasCourseInstancePropertyIri) -> bool {
		*self == HAS_COURSE_INSTANCE_PROPERTY_IRI_HTTP
			|| *self == HAS_COURSE_INSTANCE_PROPERTY_IRI_HTTPS
	}
}
pub struct HasCourseInstancePropertyIriOrLabel;
impl PartialEq<&str> for HasCourseInstancePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasCourseInstancePropertyIri || *other == HAS_COURSE_INSTANCE_PROPERTY_LABEL
	}
}
impl PartialEq<HasCourseInstancePropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasCourseInstancePropertyIriOrLabel) -> bool {
		*self == HasCourseInstancePropertyIri || *self == HAS_COURSE_INSTANCE_PROPERTY_LABEL
	}
}
