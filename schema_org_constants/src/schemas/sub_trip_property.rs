/// <https://schema.org/subTrip>
pub const SUB_TRIP_PROPERTY_IRI_HTTP: &str = "http://schema.org/subTrip";
/// <https://schema.org/subTrip>
pub const SUB_TRIP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/subTrip";
/// <https://schema.org/subTrip>
pub const SUB_TRIP_PROPERTY_LABEL: &str = "subTrip";
pub struct SubTripPropertyIri;
impl PartialEq<&str> for SubTripPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUB_TRIP_PROPERTY_IRI_HTTP || *other == SUB_TRIP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SubTripPropertyIri> for &str {
	fn eq(&self, other: &SubTripPropertyIri) -> bool {
		*self == SUB_TRIP_PROPERTY_IRI_HTTP || *self == SUB_TRIP_PROPERTY_IRI_HTTPS
	}
}
pub struct SubTripPropertyIriOrLabel;
impl PartialEq<&str> for SubTripPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SubTripPropertyIri || *other == SUB_TRIP_PROPERTY_LABEL
	}
}
impl PartialEq<SubTripPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SubTripPropertyIriOrLabel) -> bool {
		*self == SubTripPropertyIri || *self == SUB_TRIP_PROPERTY_LABEL
	}
}
