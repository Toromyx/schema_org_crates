/// <https://schema.org/numberOfBathroomsTotal>
pub const NUMBER_OF_BATHROOMS_TOTAL_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/numberOfBathroomsTotal";
/// <https://schema.org/numberOfBathroomsTotal>
pub const NUMBER_OF_BATHROOMS_TOTAL_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/numberOfBathroomsTotal";
/// <https://schema.org/numberOfBathroomsTotal>
pub const NUMBER_OF_BATHROOMS_TOTAL_PROPERTY_LABEL: &str = "numberOfBathroomsTotal";
pub struct NumberOfBathroomsTotalPropertyIri;
impl PartialEq<&str> for NumberOfBathroomsTotalPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_BATHROOMS_TOTAL_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_BATHROOMS_TOTAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfBathroomsTotalPropertyIri> for &str {
	fn eq(&self, other: &NumberOfBathroomsTotalPropertyIri) -> bool {
		*self == NUMBER_OF_BATHROOMS_TOTAL_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_BATHROOMS_TOTAL_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfBathroomsTotalPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfBathroomsTotalPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfBathroomsTotalPropertyIri
			|| *other == NUMBER_OF_BATHROOMS_TOTAL_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfBathroomsTotalPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfBathroomsTotalPropertyIriOrLabel) -> bool {
		*self == NumberOfBathroomsTotalPropertyIri
			|| *self == NUMBER_OF_BATHROOMS_TOTAL_PROPERTY_LABEL
	}
}
