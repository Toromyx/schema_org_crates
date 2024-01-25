/// <https://schema.org/partOfTrip>
pub const PART_OF_TRIP_PROPERTY_IRI_HTTP: &str = "http://schema.org/partOfTrip";
/// <https://schema.org/partOfTrip>
pub const PART_OF_TRIP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/partOfTrip";
/// <https://schema.org/partOfTrip>
pub const PART_OF_TRIP_PROPERTY_LABEL: &str = "partOfTrip";
pub struct PartOfTripPropertyIri;
impl PartialEq<&str> for PartOfTripPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PART_OF_TRIP_PROPERTY_IRI_HTTP || *other == PART_OF_TRIP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PartOfTripPropertyIri> for &str {
	fn eq(&self, other: &PartOfTripPropertyIri) -> bool {
		*self == PART_OF_TRIP_PROPERTY_IRI_HTTP || *self == PART_OF_TRIP_PROPERTY_IRI_HTTPS
	}
}
pub struct PartOfTripPropertyIriOrLabel;
impl PartialEq<&str> for PartOfTripPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PartOfTripPropertyIri || *other == PART_OF_TRIP_PROPERTY_LABEL
	}
}
impl PartialEq<PartOfTripPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PartOfTripPropertyIriOrLabel) -> bool {
		*self == PartOfTripPropertyIri || *self == PART_OF_TRIP_PROPERTY_LABEL
	}
}
