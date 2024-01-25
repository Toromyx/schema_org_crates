/// <https://schema.org/numberOfAccommodationUnits>
pub const NUMBER_OF_ACCOMMODATION_UNITS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/numberOfAccommodationUnits";
/// <https://schema.org/numberOfAccommodationUnits>
pub const NUMBER_OF_ACCOMMODATION_UNITS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/numberOfAccommodationUnits";
/// <https://schema.org/numberOfAccommodationUnits>
pub const NUMBER_OF_ACCOMMODATION_UNITS_PROPERTY_LABEL: &str = "numberOfAccommodationUnits";
pub struct NumberOfAccommodationUnitsPropertyIri;
impl PartialEq<&str> for NumberOfAccommodationUnitsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_ACCOMMODATION_UNITS_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_ACCOMMODATION_UNITS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfAccommodationUnitsPropertyIri> for &str {
	fn eq(&self, other: &NumberOfAccommodationUnitsPropertyIri) -> bool {
		*self == NUMBER_OF_ACCOMMODATION_UNITS_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_ACCOMMODATION_UNITS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfAccommodationUnitsPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfAccommodationUnitsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfAccommodationUnitsPropertyIri
			|| *other == NUMBER_OF_ACCOMMODATION_UNITS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfAccommodationUnitsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfAccommodationUnitsPropertyIriOrLabel) -> bool {
		*self == NumberOfAccommodationUnitsPropertyIri
			|| *self == NUMBER_OF_ACCOMMODATION_UNITS_PROPERTY_LABEL
	}
}
