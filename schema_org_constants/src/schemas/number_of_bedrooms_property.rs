/// <https://schema.org/numberOfBedrooms>
pub const NUMBER_OF_BEDROOMS_PROPERTY_IRI_HTTP: &str = "http://schema.org/numberOfBedrooms";
/// <https://schema.org/numberOfBedrooms>
pub const NUMBER_OF_BEDROOMS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numberOfBedrooms";
/// <https://schema.org/numberOfBedrooms>
pub const NUMBER_OF_BEDROOMS_PROPERTY_LABEL: &str = "numberOfBedrooms";
pub struct NumberOfBedroomsPropertyIri;
impl PartialEq<&str> for NumberOfBedroomsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_BEDROOMS_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_BEDROOMS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfBedroomsPropertyIri> for &str {
	fn eq(&self, other: &NumberOfBedroomsPropertyIri) -> bool {
		*self == NUMBER_OF_BEDROOMS_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_BEDROOMS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfBedroomsPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfBedroomsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfBedroomsPropertyIri || *other == NUMBER_OF_BEDROOMS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfBedroomsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfBedroomsPropertyIriOrLabel) -> bool {
		*self == NumberOfBedroomsPropertyIri || *self == NUMBER_OF_BEDROOMS_PROPERTY_LABEL
	}
}
