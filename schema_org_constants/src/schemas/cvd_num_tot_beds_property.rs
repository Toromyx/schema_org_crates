/// <https://schema.org/cvdNumTotBeds>
pub const CVD_NUM_TOT_BEDS_PROPERTY_IRI_HTTP: &str = "http://schema.org/cvdNumTotBeds";
/// <https://schema.org/cvdNumTotBeds>
pub const CVD_NUM_TOT_BEDS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cvdNumTotBeds";
/// <https://schema.org/cvdNumTotBeds>
pub const CVD_NUM_TOT_BEDS_PROPERTY_LABEL: &str = "cvdNumTotBeds";
pub struct CvdNumTotBedsPropertyIri;
impl PartialEq<&str> for CvdNumTotBedsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_NUM_TOT_BEDS_PROPERTY_IRI_HTTP
			|| *other == CVD_NUM_TOT_BEDS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdNumTotBedsPropertyIri> for &str {
	fn eq(&self, other: &CvdNumTotBedsPropertyIri) -> bool {
		*self == CVD_NUM_TOT_BEDS_PROPERTY_IRI_HTTP || *self == CVD_NUM_TOT_BEDS_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdNumTotBedsPropertyIriOrLabel;
impl PartialEq<&str> for CvdNumTotBedsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdNumTotBedsPropertyIri || *other == CVD_NUM_TOT_BEDS_PROPERTY_LABEL
	}
}
impl PartialEq<CvdNumTotBedsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdNumTotBedsPropertyIriOrLabel) -> bool {
		*self == CvdNumTotBedsPropertyIri || *self == CVD_NUM_TOT_BEDS_PROPERTY_LABEL
	}
}
