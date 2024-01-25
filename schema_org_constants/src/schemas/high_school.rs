/// <https://schema.org/HighSchool>
pub const HIGH_SCHOOL_IRI_HTTP: &str = "http://schema.org/HighSchool";
/// <https://schema.org/HighSchool>
pub const HIGH_SCHOOL_IRI_HTTPS: &str = "https://schema.org/HighSchool";
/// <https://schema.org/HighSchool>
pub const HIGH_SCHOOL_LABEL: &str = "HighSchool";
pub struct HighSchoolIri;
impl PartialEq<&str> for HighSchoolIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HIGH_SCHOOL_IRI_HTTP || *other == HIGH_SCHOOL_IRI_HTTPS
	}
}
impl PartialEq<HighSchoolIri> for &str {
	fn eq(&self, other: &HighSchoolIri) -> bool {
		*self == HIGH_SCHOOL_IRI_HTTP || *self == HIGH_SCHOOL_IRI_HTTPS
	}
}
pub struct HighSchoolIriOrLabel;
impl PartialEq<&str> for HighSchoolIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HighSchoolIri || *other == HIGH_SCHOOL_LABEL
	}
}
impl PartialEq<HighSchoolIriOrLabel> for &str {
	fn eq(&self, other: &HighSchoolIriOrLabel) -> bool {
		*self == HighSchoolIri || *self == HIGH_SCHOOL_LABEL
	}
}
