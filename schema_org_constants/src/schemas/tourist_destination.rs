/// <https://schema.org/TouristDestination>
pub const TOURIST_DESTINATION_IRI_HTTP: &str = "http://schema.org/TouristDestination";
/// <https://schema.org/TouristDestination>
pub const TOURIST_DESTINATION_IRI_HTTPS: &str = "https://schema.org/TouristDestination";
/// <https://schema.org/TouristDestination>
pub const TOURIST_DESTINATION_LABEL: &str = "TouristDestination";
pub struct TouristDestinationIri;
impl PartialEq<&str> for TouristDestinationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOURIST_DESTINATION_IRI_HTTP || *other == TOURIST_DESTINATION_IRI_HTTPS
	}
}
impl PartialEq<TouristDestinationIri> for &str {
	fn eq(&self, other: &TouristDestinationIri) -> bool {
		*self == TOURIST_DESTINATION_IRI_HTTP || *self == TOURIST_DESTINATION_IRI_HTTPS
	}
}
pub struct TouristDestinationIriOrLabel;
impl PartialEq<&str> for TouristDestinationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TouristDestinationIri || *other == TOURIST_DESTINATION_LABEL
	}
}
impl PartialEq<TouristDestinationIriOrLabel> for &str {
	fn eq(&self, other: &TouristDestinationIriOrLabel) -> bool {
		*self == TouristDestinationIri || *self == TOURIST_DESTINATION_LABEL
	}
}
