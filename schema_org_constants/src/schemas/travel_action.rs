/// <https://schema.org/TravelAction>
pub const TRAVEL_ACTION_IRI_HTTP: &str = "http://schema.org/TravelAction";
/// <https://schema.org/TravelAction>
pub const TRAVEL_ACTION_IRI_HTTPS: &str = "https://schema.org/TravelAction";
/// <https://schema.org/TravelAction>
pub const TRAVEL_ACTION_LABEL: &str = "TravelAction";
pub struct TravelActionIri;
impl PartialEq<&str> for TravelActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRAVEL_ACTION_IRI_HTTP || *other == TRAVEL_ACTION_IRI_HTTPS
	}
}
impl PartialEq<TravelActionIri> for &str {
	fn eq(&self, other: &TravelActionIri) -> bool {
		*self == TRAVEL_ACTION_IRI_HTTP || *self == TRAVEL_ACTION_IRI_HTTPS
	}
}
pub struct TravelActionIriOrLabel;
impl PartialEq<&str> for TravelActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TravelActionIri || *other == TRAVEL_ACTION_LABEL
	}
}
impl PartialEq<TravelActionIriOrLabel> for &str {
	fn eq(&self, other: &TravelActionIriOrLabel) -> bool {
		*self == TravelActionIri || *self == TRAVEL_ACTION_LABEL
	}
}
