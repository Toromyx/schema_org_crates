/// <https://schema.org/numberOfAirbags>
pub const NUMBER_OF_AIRBAGS_PROPERTY_IRI_HTTP: &str = "http://schema.org/numberOfAirbags";
/// <https://schema.org/numberOfAirbags>
pub const NUMBER_OF_AIRBAGS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numberOfAirbags";
/// <https://schema.org/numberOfAirbags>
pub const NUMBER_OF_AIRBAGS_PROPERTY_LABEL: &str = "numberOfAirbags";
pub struct NumberOfAirbagsPropertyIri;
impl PartialEq<&str> for NumberOfAirbagsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_AIRBAGS_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_AIRBAGS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfAirbagsPropertyIri> for &str {
	fn eq(&self, other: &NumberOfAirbagsPropertyIri) -> bool {
		*self == NUMBER_OF_AIRBAGS_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_AIRBAGS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfAirbagsPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfAirbagsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfAirbagsPropertyIri || *other == NUMBER_OF_AIRBAGS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfAirbagsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfAirbagsPropertyIriOrLabel) -> bool {
		*self == NumberOfAirbagsPropertyIri || *self == NUMBER_OF_AIRBAGS_PROPERTY_LABEL
	}
}
