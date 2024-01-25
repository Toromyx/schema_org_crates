/// <https://schema.org/itinerary>
pub const ITINERARY_PROPERTY_IRI_HTTP: &str = "http://schema.org/itinerary";
/// <https://schema.org/itinerary>
pub const ITINERARY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/itinerary";
/// <https://schema.org/itinerary>
pub const ITINERARY_PROPERTY_LABEL: &str = "itinerary";
pub struct ItineraryPropertyIri;
impl PartialEq<&str> for ItineraryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITINERARY_PROPERTY_IRI_HTTP || *other == ITINERARY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ItineraryPropertyIri> for &str {
	fn eq(&self, other: &ItineraryPropertyIri) -> bool {
		*self == ITINERARY_PROPERTY_IRI_HTTP || *self == ITINERARY_PROPERTY_IRI_HTTPS
	}
}
pub struct ItineraryPropertyIriOrLabel;
impl PartialEq<&str> for ItineraryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItineraryPropertyIri || *other == ITINERARY_PROPERTY_LABEL
	}
}
impl PartialEq<ItineraryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ItineraryPropertyIriOrLabel) -> bool {
		*self == ItineraryPropertyIri || *self == ITINERARY_PROPERTY_LABEL
	}
}
