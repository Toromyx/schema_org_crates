/// <https://schema.org/SchoolDistrict>
pub const SCHOOL_DISTRICT_IRI_HTTP: &str = "http://schema.org/SchoolDistrict";
/// <https://schema.org/SchoolDistrict>
pub const SCHOOL_DISTRICT_IRI_HTTPS: &str = "https://schema.org/SchoolDistrict";
/// <https://schema.org/SchoolDistrict>
pub const SCHOOL_DISTRICT_LABEL: &str = "SchoolDistrict";
pub struct SchoolDistrictIri;
impl PartialEq<&str> for SchoolDistrictIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCHOOL_DISTRICT_IRI_HTTP || *other == SCHOOL_DISTRICT_IRI_HTTPS
	}
}
impl PartialEq<SchoolDistrictIri> for &str {
	fn eq(&self, other: &SchoolDistrictIri) -> bool {
		*self == SCHOOL_DISTRICT_IRI_HTTP || *self == SCHOOL_DISTRICT_IRI_HTTPS
	}
}
pub struct SchoolDistrictIriOrLabel;
impl PartialEq<&str> for SchoolDistrictIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SchoolDistrictIri || *other == SCHOOL_DISTRICT_LABEL
	}
}
impl PartialEq<SchoolDistrictIriOrLabel> for &str {
	fn eq(&self, other: &SchoolDistrictIriOrLabel) -> bool {
		*self == SchoolDistrictIri || *self == SCHOOL_DISTRICT_LABEL
	}
}
