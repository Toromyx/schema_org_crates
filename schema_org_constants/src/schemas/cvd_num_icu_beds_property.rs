/// <https://schema.org/cvdNumICUBeds>
pub const CVD_NUM_ICU_BEDS_PROPERTY_IRI_HTTP: &str = "http://schema.org/cvdNumICUBeds";
/// <https://schema.org/cvdNumICUBeds>
pub const CVD_NUM_ICU_BEDS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cvdNumICUBeds";
/// <https://schema.org/cvdNumICUBeds>
pub const CVD_NUM_ICU_BEDS_PROPERTY_LABEL: &str = "cvdNumICUBeds";
pub struct CvdNumIcuBedsPropertyIri;
impl PartialEq<&str> for CvdNumIcuBedsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_NUM_ICU_BEDS_PROPERTY_IRI_HTTP
			|| *other == CVD_NUM_ICU_BEDS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdNumIcuBedsPropertyIri> for &str {
	fn eq(&self, other: &CvdNumIcuBedsPropertyIri) -> bool {
		*self == CVD_NUM_ICU_BEDS_PROPERTY_IRI_HTTP || *self == CVD_NUM_ICU_BEDS_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdNumIcuBedsPropertyIriOrLabel;
impl PartialEq<&str> for CvdNumIcuBedsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdNumIcuBedsPropertyIri || *other == CVD_NUM_ICU_BEDS_PROPERTY_LABEL
	}
}
impl PartialEq<CvdNumIcuBedsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdNumIcuBedsPropertyIriOrLabel) -> bool {
		*self == CvdNumIcuBedsPropertyIri || *self == CVD_NUM_ICU_BEDS_PROPERTY_LABEL
	}
}
