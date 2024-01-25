/// <https://schema.org/cvdNumBedsOcc>
pub const CVD_NUM_BEDS_OCC_PROPERTY_IRI_HTTP: &str = "http://schema.org/cvdNumBedsOcc";
/// <https://schema.org/cvdNumBedsOcc>
pub const CVD_NUM_BEDS_OCC_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cvdNumBedsOcc";
/// <https://schema.org/cvdNumBedsOcc>
pub const CVD_NUM_BEDS_OCC_PROPERTY_LABEL: &str = "cvdNumBedsOcc";
pub struct CvdNumBedsOccPropertyIri;
impl PartialEq<&str> for CvdNumBedsOccPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_NUM_BEDS_OCC_PROPERTY_IRI_HTTP
			|| *other == CVD_NUM_BEDS_OCC_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdNumBedsOccPropertyIri> for &str {
	fn eq(&self, other: &CvdNumBedsOccPropertyIri) -> bool {
		*self == CVD_NUM_BEDS_OCC_PROPERTY_IRI_HTTP || *self == CVD_NUM_BEDS_OCC_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdNumBedsOccPropertyIriOrLabel;
impl PartialEq<&str> for CvdNumBedsOccPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdNumBedsOccPropertyIri || *other == CVD_NUM_BEDS_OCC_PROPERTY_LABEL
	}
}
impl PartialEq<CvdNumBedsOccPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdNumBedsOccPropertyIriOrLabel) -> bool {
		*self == CvdNumBedsOccPropertyIri || *self == CVD_NUM_BEDS_OCC_PROPERTY_LABEL
	}
}
