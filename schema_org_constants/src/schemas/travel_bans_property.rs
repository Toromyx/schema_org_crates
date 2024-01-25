/// <https://schema.org/travelBans>
pub const TRAVEL_BANS_PROPERTY_IRI_HTTP: &str = "http://schema.org/travelBans";
/// <https://schema.org/travelBans>
pub const TRAVEL_BANS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/travelBans";
/// <https://schema.org/travelBans>
pub const TRAVEL_BANS_PROPERTY_LABEL: &str = "travelBans";
pub struct TravelBansPropertyIri;
impl PartialEq<&str> for TravelBansPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRAVEL_BANS_PROPERTY_IRI_HTTP || *other == TRAVEL_BANS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TravelBansPropertyIri> for &str {
	fn eq(&self, other: &TravelBansPropertyIri) -> bool {
		*self == TRAVEL_BANS_PROPERTY_IRI_HTTP || *self == TRAVEL_BANS_PROPERTY_IRI_HTTPS
	}
}
pub struct TravelBansPropertyIriOrLabel;
impl PartialEq<&str> for TravelBansPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TravelBansPropertyIri || *other == TRAVEL_BANS_PROPERTY_LABEL
	}
}
impl PartialEq<TravelBansPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TravelBansPropertyIriOrLabel) -> bool {
		*self == TravelBansPropertyIri || *self == TRAVEL_BANS_PROPERTY_LABEL
	}
}
