/// <https://schema.org/cvdNumICUBedsOcc>
pub const CVD_NUM_ICU_BEDS_OCC_PROPERTY_IRI_HTTP: &str = "http://schema.org/cvdNumICUBedsOcc";
/// <https://schema.org/cvdNumICUBedsOcc>
pub const CVD_NUM_ICU_BEDS_OCC_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cvdNumICUBedsOcc";
/// <https://schema.org/cvdNumICUBedsOcc>
pub const CVD_NUM_ICU_BEDS_OCC_PROPERTY_LABEL: &str = "cvdNumICUBedsOcc";
pub struct CvdNumIcuBedsOccPropertyIri;
impl PartialEq<&str> for CvdNumIcuBedsOccPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_NUM_ICU_BEDS_OCC_PROPERTY_IRI_HTTP
			|| *other == CVD_NUM_ICU_BEDS_OCC_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdNumIcuBedsOccPropertyIri> for &str {
	fn eq(&self, other: &CvdNumIcuBedsOccPropertyIri) -> bool {
		*self == CVD_NUM_ICU_BEDS_OCC_PROPERTY_IRI_HTTP
			|| *self == CVD_NUM_ICU_BEDS_OCC_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdNumIcuBedsOccPropertyIriOrLabel;
impl PartialEq<&str> for CvdNumIcuBedsOccPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdNumIcuBedsOccPropertyIri || *other == CVD_NUM_ICU_BEDS_OCC_PROPERTY_LABEL
	}
}
impl PartialEq<CvdNumIcuBedsOccPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdNumIcuBedsOccPropertyIriOrLabel) -> bool {
		*self == CvdNumIcuBedsOccPropertyIri || *self == CVD_NUM_ICU_BEDS_OCC_PROPERTY_LABEL
	}
}
