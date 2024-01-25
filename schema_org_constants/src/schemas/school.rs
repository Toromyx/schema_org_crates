/// <https://schema.org/School>
pub const SCHOOL_IRI_HTTP: &str = "http://schema.org/School";
/// <https://schema.org/School>
pub const SCHOOL_IRI_HTTPS: &str = "https://schema.org/School";
/// <https://schema.org/School>
pub const SCHOOL_LABEL: &str = "School";
pub struct SchoolIri;
impl PartialEq<&str> for SchoolIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCHOOL_IRI_HTTP || *other == SCHOOL_IRI_HTTPS
	}
}
impl PartialEq<SchoolIri> for &str {
	fn eq(&self, other: &SchoolIri) -> bool {
		*self == SCHOOL_IRI_HTTP || *self == SCHOOL_IRI_HTTPS
	}
}
pub struct SchoolIriOrLabel;
impl PartialEq<&str> for SchoolIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SchoolIri || *other == SCHOOL_LABEL
	}
}
impl PartialEq<SchoolIriOrLabel> for &str {
	fn eq(&self, other: &SchoolIriOrLabel) -> bool {
		*self == SchoolIri || *self == SCHOOL_LABEL
	}
}
