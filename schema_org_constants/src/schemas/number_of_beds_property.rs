/// <https://schema.org/numberOfBeds>
pub const NUMBER_OF_BEDS_PROPERTY_IRI_HTTP: &str = "http://schema.org/numberOfBeds";
/// <https://schema.org/numberOfBeds>
pub const NUMBER_OF_BEDS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numberOfBeds";
/// <https://schema.org/numberOfBeds>
pub const NUMBER_OF_BEDS_PROPERTY_LABEL: &str = "numberOfBeds";
pub struct NumberOfBedsPropertyIri;
impl PartialEq<&str> for NumberOfBedsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_BEDS_PROPERTY_IRI_HTTP || *other == NUMBER_OF_BEDS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfBedsPropertyIri> for &str {
	fn eq(&self, other: &NumberOfBedsPropertyIri) -> bool {
		*self == NUMBER_OF_BEDS_PROPERTY_IRI_HTTP || *self == NUMBER_OF_BEDS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfBedsPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfBedsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfBedsPropertyIri || *other == NUMBER_OF_BEDS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfBedsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfBedsPropertyIriOrLabel) -> bool {
		*self == NumberOfBedsPropertyIri || *self == NUMBER_OF_BEDS_PROPERTY_LABEL
	}
}
