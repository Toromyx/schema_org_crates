/// <https://schema.org/MiddleSchool>
pub const MIDDLE_SCHOOL_IRI_HTTP: &str = "http://schema.org/MiddleSchool";
/// <https://schema.org/MiddleSchool>
pub const MIDDLE_SCHOOL_IRI_HTTPS: &str = "https://schema.org/MiddleSchool";
/// <https://schema.org/MiddleSchool>
pub const MIDDLE_SCHOOL_LABEL: &str = "MiddleSchool";
pub struct MiddleSchoolIri;
impl PartialEq<&str> for MiddleSchoolIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MIDDLE_SCHOOL_IRI_HTTP || *other == MIDDLE_SCHOOL_IRI_HTTPS
	}
}
impl PartialEq<MiddleSchoolIri> for &str {
	fn eq(&self, other: &MiddleSchoolIri) -> bool {
		*self == MIDDLE_SCHOOL_IRI_HTTP || *self == MIDDLE_SCHOOL_IRI_HTTPS
	}
}
pub struct MiddleSchoolIriOrLabel;
impl PartialEq<&str> for MiddleSchoolIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MiddleSchoolIri || *other == MIDDLE_SCHOOL_LABEL
	}
}
impl PartialEq<MiddleSchoolIriOrLabel> for &str {
	fn eq(&self, other: &MiddleSchoolIriOrLabel) -> bool {
		*self == MiddleSchoolIri || *self == MIDDLE_SCHOOL_LABEL
	}
}
