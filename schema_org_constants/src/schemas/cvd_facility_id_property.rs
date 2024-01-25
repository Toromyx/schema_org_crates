/// <https://schema.org/cvdFacilityId>
pub const CVD_FACILITY_ID_PROPERTY_IRI_HTTP: &str = "http://schema.org/cvdFacilityId";
/// <https://schema.org/cvdFacilityId>
pub const CVD_FACILITY_ID_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cvdFacilityId";
/// <https://schema.org/cvdFacilityId>
pub const CVD_FACILITY_ID_PROPERTY_LABEL: &str = "cvdFacilityId";
pub struct CvdFacilityIdPropertyIri;
impl PartialEq<&str> for CvdFacilityIdPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_FACILITY_ID_PROPERTY_IRI_HTTP || *other == CVD_FACILITY_ID_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdFacilityIdPropertyIri> for &str {
	fn eq(&self, other: &CvdFacilityIdPropertyIri) -> bool {
		*self == CVD_FACILITY_ID_PROPERTY_IRI_HTTP || *self == CVD_FACILITY_ID_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdFacilityIdPropertyIriOrLabel;
impl PartialEq<&str> for CvdFacilityIdPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdFacilityIdPropertyIri || *other == CVD_FACILITY_ID_PROPERTY_LABEL
	}
}
impl PartialEq<CvdFacilityIdPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdFacilityIdPropertyIriOrLabel) -> bool {
		*self == CvdFacilityIdPropertyIri || *self == CVD_FACILITY_ID_PROPERTY_LABEL
	}
}
