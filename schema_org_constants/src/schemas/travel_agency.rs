/// <https://schema.org/TravelAgency>
pub const TRAVEL_AGENCY_IRI_HTTP: &str = "http://schema.org/TravelAgency";
/// <https://schema.org/TravelAgency>
pub const TRAVEL_AGENCY_IRI_HTTPS: &str = "https://schema.org/TravelAgency";
/// <https://schema.org/TravelAgency>
pub const TRAVEL_AGENCY_LABEL: &str = "TravelAgency";
pub struct TravelAgencyIri;
impl PartialEq<&str> for TravelAgencyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRAVEL_AGENCY_IRI_HTTP || *other == TRAVEL_AGENCY_IRI_HTTPS
	}
}
impl PartialEq<TravelAgencyIri> for &str {
	fn eq(&self, other: &TravelAgencyIri) -> bool {
		*self == TRAVEL_AGENCY_IRI_HTTP || *self == TRAVEL_AGENCY_IRI_HTTPS
	}
}
pub struct TravelAgencyIriOrLabel;
impl PartialEq<&str> for TravelAgencyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TravelAgencyIri || *other == TRAVEL_AGENCY_LABEL
	}
}
impl PartialEq<TravelAgencyIriOrLabel> for &str {
	fn eq(&self, other: &TravelAgencyIriOrLabel) -> bool {
		*self == TravelAgencyIri || *self == TRAVEL_AGENCY_LABEL
	}
}
