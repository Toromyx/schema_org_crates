/// <https://schema.org/numberOfPartialBathrooms>
pub const NUMBER_OF_PARTIAL_BATHROOMS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/numberOfPartialBathrooms";
/// <https://schema.org/numberOfPartialBathrooms>
pub const NUMBER_OF_PARTIAL_BATHROOMS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/numberOfPartialBathrooms";
/// <https://schema.org/numberOfPartialBathrooms>
pub const NUMBER_OF_PARTIAL_BATHROOMS_PROPERTY_LABEL: &str = "numberOfPartialBathrooms";
pub struct NumberOfPartialBathroomsPropertyIri;
impl PartialEq<&str> for NumberOfPartialBathroomsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_PARTIAL_BATHROOMS_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_PARTIAL_BATHROOMS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfPartialBathroomsPropertyIri> for &str {
	fn eq(&self, other: &NumberOfPartialBathroomsPropertyIri) -> bool {
		*self == NUMBER_OF_PARTIAL_BATHROOMS_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_PARTIAL_BATHROOMS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfPartialBathroomsPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfPartialBathroomsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfPartialBathroomsPropertyIri
			|| *other == NUMBER_OF_PARTIAL_BATHROOMS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfPartialBathroomsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfPartialBathroomsPropertyIriOrLabel) -> bool {
		*self == NumberOfPartialBathroomsPropertyIri
			|| *self == NUMBER_OF_PARTIAL_BATHROOMS_PROPERTY_LABEL
	}
}
