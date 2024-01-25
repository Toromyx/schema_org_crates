/// <https://schema.org/cvdNumBeds>
pub const CVD_NUM_BEDS_PROPERTY_IRI_HTTP: &str = "http://schema.org/cvdNumBeds";
/// <https://schema.org/cvdNumBeds>
pub const CVD_NUM_BEDS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cvdNumBeds";
/// <https://schema.org/cvdNumBeds>
pub const CVD_NUM_BEDS_PROPERTY_LABEL: &str = "cvdNumBeds";
pub struct CvdNumBedsPropertyIri;
impl PartialEq<&str> for CvdNumBedsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_NUM_BEDS_PROPERTY_IRI_HTTP || *other == CVD_NUM_BEDS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdNumBedsPropertyIri> for &str {
	fn eq(&self, other: &CvdNumBedsPropertyIri) -> bool {
		*self == CVD_NUM_BEDS_PROPERTY_IRI_HTTP || *self == CVD_NUM_BEDS_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdNumBedsPropertyIriOrLabel;
impl PartialEq<&str> for CvdNumBedsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdNumBedsPropertyIri || *other == CVD_NUM_BEDS_PROPERTY_LABEL
	}
}
impl PartialEq<CvdNumBedsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdNumBedsPropertyIriOrLabel) -> bool {
		*self == CvdNumBedsPropertyIri || *self == CVD_NUM_BEDS_PROPERTY_LABEL
	}
}
