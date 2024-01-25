/// <https://schema.org/cvdNumVent>
pub const CVD_NUM_VENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/cvdNumVent";
/// <https://schema.org/cvdNumVent>
pub const CVD_NUM_VENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cvdNumVent";
/// <https://schema.org/cvdNumVent>
pub const CVD_NUM_VENT_PROPERTY_LABEL: &str = "cvdNumVent";
pub struct CvdNumVentPropertyIri;
impl PartialEq<&str> for CvdNumVentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_NUM_VENT_PROPERTY_IRI_HTTP || *other == CVD_NUM_VENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdNumVentPropertyIri> for &str {
	fn eq(&self, other: &CvdNumVentPropertyIri) -> bool {
		*self == CVD_NUM_VENT_PROPERTY_IRI_HTTP || *self == CVD_NUM_VENT_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdNumVentPropertyIriOrLabel;
impl PartialEq<&str> for CvdNumVentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdNumVentPropertyIri || *other == CVD_NUM_VENT_PROPERTY_LABEL
	}
}
impl PartialEq<CvdNumVentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdNumVentPropertyIriOrLabel) -> bool {
		*self == CvdNumVentPropertyIri || *self == CVD_NUM_VENT_PROPERTY_LABEL
	}
}
