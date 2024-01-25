/// <https://schema.org/cvdNumC19Died>
pub const CVD_NUM_C_19_DIED_PROPERTY_IRI_HTTP: &str = "http://schema.org/cvdNumC19Died";
/// <https://schema.org/cvdNumC19Died>
pub const CVD_NUM_C_19_DIED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cvdNumC19Died";
/// <https://schema.org/cvdNumC19Died>
pub const CVD_NUM_C_19_DIED_PROPERTY_LABEL: &str = "cvdNumC19Died";
pub struct CvdNumC19DiedPropertyIri;
impl PartialEq<&str> for CvdNumC19DiedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_NUM_C_19_DIED_PROPERTY_IRI_HTTP
			|| *other == CVD_NUM_C_19_DIED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdNumC19DiedPropertyIri> for &str {
	fn eq(&self, other: &CvdNumC19DiedPropertyIri) -> bool {
		*self == CVD_NUM_C_19_DIED_PROPERTY_IRI_HTTP
			|| *self == CVD_NUM_C_19_DIED_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdNumC19DiedPropertyIriOrLabel;
impl PartialEq<&str> for CvdNumC19DiedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdNumC19DiedPropertyIri || *other == CVD_NUM_C_19_DIED_PROPERTY_LABEL
	}
}
impl PartialEq<CvdNumC19DiedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdNumC19DiedPropertyIriOrLabel) -> bool {
		*self == CvdNumC19DiedPropertyIri || *self == CVD_NUM_C_19_DIED_PROPERTY_LABEL
	}
}
