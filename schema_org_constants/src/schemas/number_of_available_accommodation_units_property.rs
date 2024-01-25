/// <https://schema.org/numberOfAvailableAccommodationUnits>
pub const NUMBER_OF_AVAILABLE_ACCOMMODATION_UNITS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/numberOfAvailableAccommodationUnits";
/// <https://schema.org/numberOfAvailableAccommodationUnits>
pub const NUMBER_OF_AVAILABLE_ACCOMMODATION_UNITS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/numberOfAvailableAccommodationUnits";
/// <https://schema.org/numberOfAvailableAccommodationUnits>
pub const NUMBER_OF_AVAILABLE_ACCOMMODATION_UNITS_PROPERTY_LABEL: &str =
	"numberOfAvailableAccommodationUnits";
pub struct NumberOfAvailableAccommodationUnitsPropertyIri;
impl PartialEq<&str> for NumberOfAvailableAccommodationUnitsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_AVAILABLE_ACCOMMODATION_UNITS_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_AVAILABLE_ACCOMMODATION_UNITS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfAvailableAccommodationUnitsPropertyIri> for &str {
	fn eq(&self, other: &NumberOfAvailableAccommodationUnitsPropertyIri) -> bool {
		*self == NUMBER_OF_AVAILABLE_ACCOMMODATION_UNITS_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_AVAILABLE_ACCOMMODATION_UNITS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfAvailableAccommodationUnitsPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfAvailableAccommodationUnitsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfAvailableAccommodationUnitsPropertyIri
			|| *other == NUMBER_OF_AVAILABLE_ACCOMMODATION_UNITS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfAvailableAccommodationUnitsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfAvailableAccommodationUnitsPropertyIriOrLabel) -> bool {
		*self == NumberOfAvailableAccommodationUnitsPropertyIri
			|| *self == NUMBER_OF_AVAILABLE_ACCOMMODATION_UNITS_PROPERTY_LABEL
	}
}
