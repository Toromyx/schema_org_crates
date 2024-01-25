/// <https://schema.org/CovidTestingFacility>
pub const COVID_TESTING_FACILITY_IRI_HTTP: &str = "http://schema.org/CovidTestingFacility";
/// <https://schema.org/CovidTestingFacility>
pub const COVID_TESTING_FACILITY_IRI_HTTPS: &str = "https://schema.org/CovidTestingFacility";
/// <https://schema.org/CovidTestingFacility>
pub const COVID_TESTING_FACILITY_LABEL: &str = "CovidTestingFacility";
pub struct CovidTestingFacilityIri;
impl PartialEq<&str> for CovidTestingFacilityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COVID_TESTING_FACILITY_IRI_HTTP || *other == COVID_TESTING_FACILITY_IRI_HTTPS
	}
}
impl PartialEq<CovidTestingFacilityIri> for &str {
	fn eq(&self, other: &CovidTestingFacilityIri) -> bool {
		*self == COVID_TESTING_FACILITY_IRI_HTTP || *self == COVID_TESTING_FACILITY_IRI_HTTPS
	}
}
pub struct CovidTestingFacilityIriOrLabel;
impl PartialEq<&str> for CovidTestingFacilityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CovidTestingFacilityIri || *other == COVID_TESTING_FACILITY_LABEL
	}
}
impl PartialEq<CovidTestingFacilityIriOrLabel> for &str {
	fn eq(&self, other: &CovidTestingFacilityIriOrLabel) -> bool {
		*self == CovidTestingFacilityIri || *self == COVID_TESTING_FACILITY_LABEL
	}
}
