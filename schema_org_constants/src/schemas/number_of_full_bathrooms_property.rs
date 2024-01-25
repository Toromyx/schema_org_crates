/// <https://schema.org/numberOfFullBathrooms>
pub const NUMBER_OF_FULL_BATHROOMS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/numberOfFullBathrooms";
/// <https://schema.org/numberOfFullBathrooms>
pub const NUMBER_OF_FULL_BATHROOMS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/numberOfFullBathrooms";
/// <https://schema.org/numberOfFullBathrooms>
pub const NUMBER_OF_FULL_BATHROOMS_PROPERTY_LABEL: &str = "numberOfFullBathrooms";
pub struct NumberOfFullBathroomsPropertyIri;
impl PartialEq<&str> for NumberOfFullBathroomsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_FULL_BATHROOMS_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_FULL_BATHROOMS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfFullBathroomsPropertyIri> for &str {
	fn eq(&self, other: &NumberOfFullBathroomsPropertyIri) -> bool {
		*self == NUMBER_OF_FULL_BATHROOMS_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_FULL_BATHROOMS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfFullBathroomsPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfFullBathroomsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfFullBathroomsPropertyIri
			|| *other == NUMBER_OF_FULL_BATHROOMS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfFullBathroomsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfFullBathroomsPropertyIriOrLabel) -> bool {
		*self == NumberOfFullBathroomsPropertyIri
			|| *self == NUMBER_OF_FULL_BATHROOMS_PROPERTY_LABEL
	}
}
