/// <https://schema.org/cvdFacilityCounty>
pub const CVD_FACILITY_COUNTY_PROPERTY_IRI_HTTP: &str = "http://schema.org/cvdFacilityCounty";
/// <https://schema.org/cvdFacilityCounty>
pub const CVD_FACILITY_COUNTY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cvdFacilityCounty";
/// <https://schema.org/cvdFacilityCounty>
pub const CVD_FACILITY_COUNTY_PROPERTY_LABEL: &str = "cvdFacilityCounty";
pub struct CvdFacilityCountyPropertyIri;
impl PartialEq<&str> for CvdFacilityCountyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CVD_FACILITY_COUNTY_PROPERTY_IRI_HTTP
			|| *other == CVD_FACILITY_COUNTY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CvdFacilityCountyPropertyIri> for &str {
	fn eq(&self, other: &CvdFacilityCountyPropertyIri) -> bool {
		*self == CVD_FACILITY_COUNTY_PROPERTY_IRI_HTTP
			|| *self == CVD_FACILITY_COUNTY_PROPERTY_IRI_HTTPS
	}
}
pub struct CvdFacilityCountyPropertyIriOrLabel;
impl PartialEq<&str> for CvdFacilityCountyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CvdFacilityCountyPropertyIri || *other == CVD_FACILITY_COUNTY_PROPERTY_LABEL
	}
}
impl PartialEq<CvdFacilityCountyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CvdFacilityCountyPropertyIriOrLabel) -> bool {
		*self == CvdFacilityCountyPropertyIri || *self == CVD_FACILITY_COUNTY_PROPERTY_LABEL
	}
}
