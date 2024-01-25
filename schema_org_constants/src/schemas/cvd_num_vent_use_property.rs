/// <https://schema.org/cvdNumVentUse>
pub const CVD_NUM_VENT_USE_PROPERTY_IRI_HTTP: &str = "http://schema.org/cvdNumVentUse";
/// <https://schema.org/cvdNumVentUse>
pub const CVD_NUM_VENT_USE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cvdNumVentUse";
/// <https://schema.org/cvdNumVentUse>
pub const CVD_NUM_VENT_USE_PROPERTY_LABEL: &str = "cvdNumVentUse";
pub struct CvdNumVentUsePropertyIri;
impl PartialEq<&str> for CvdNumVentUsePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_NUM_VENT_USE_PROPERTY_IRI_HTTP
			|| *other == CVD_NUM_VENT_USE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdNumVentUsePropertyIri> for &str {
	fn eq(&self, other: &CvdNumVentUsePropertyIri) -> bool {
		*self == CVD_NUM_VENT_USE_PROPERTY_IRI_HTTP || *self == CVD_NUM_VENT_USE_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdNumVentUsePropertyIriOrLabel;
impl PartialEq<&str> for CvdNumVentUsePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdNumVentUsePropertyIri || *other == CVD_NUM_VENT_USE_PROPERTY_LABEL
	}
}
impl PartialEq<CvdNumVentUsePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdNumVentUsePropertyIriOrLabel) -> bool {
		*self == CvdNumVentUsePropertyIri || *self == CVD_NUM_VENT_USE_PROPERTY_LABEL
	}
}
