/// <https://schema.org/schoolClosuresInfo>
pub const SCHOOL_CLOSURES_INFO_PROPERTY_IRI_HTTP: &str = "http://schema.org/schoolClosuresInfo";
/// <https://schema.org/schoolClosuresInfo>
pub const SCHOOL_CLOSURES_INFO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/schoolClosuresInfo";
/// <https://schema.org/schoolClosuresInfo>
pub const SCHOOL_CLOSURES_INFO_PROPERTY_LABEL: &str = "schoolClosuresInfo";
pub struct SchoolClosuresInfoPropertyIri;
impl PartialEq<&str> for SchoolClosuresInfoPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCHOOL_CLOSURES_INFO_PROPERTY_IRI_HTTP
			|| *other == SCHOOL_CLOSURES_INFO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SchoolClosuresInfoPropertyIri> for &str {
	fn eq(&self, other: &SchoolClosuresInfoPropertyIri) -> bool {
		*self == SCHOOL_CLOSURES_INFO_PROPERTY_IRI_HTTP
			|| *self == SCHOOL_CLOSURES_INFO_PROPERTY_IRI_HTTPS
	}
}
pub struct SchoolClosuresInfoPropertyIriOrLabel;
impl PartialEq<&str> for SchoolClosuresInfoPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SchoolClosuresInfoPropertyIri || *other == SCHOOL_CLOSURES_INFO_PROPERTY_LABEL
	}
}
impl PartialEq<SchoolClosuresInfoPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SchoolClosuresInfoPropertyIriOrLabel) -> bool {
		*self == SchoolClosuresInfoPropertyIri || *self == SCHOOL_CLOSURES_INFO_PROPERTY_LABEL
	}
}
