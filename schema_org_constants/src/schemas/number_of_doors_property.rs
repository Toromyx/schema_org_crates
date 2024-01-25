/// <https://schema.org/numberOfDoors>
pub const NUMBER_OF_DOORS_PROPERTY_IRI_HTTP: &str = "http://schema.org/numberOfDoors";
/// <https://schema.org/numberOfDoors>
pub const NUMBER_OF_DOORS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numberOfDoors";
/// <https://schema.org/numberOfDoors>
pub const NUMBER_OF_DOORS_PROPERTY_LABEL: &str = "numberOfDoors";
pub struct NumberOfDoorsPropertyIri;
impl PartialEq<&str> for NumberOfDoorsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_DOORS_PROPERTY_IRI_HTTP || *other == NUMBER_OF_DOORS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfDoorsPropertyIri> for &str {
	fn eq(&self, other: &NumberOfDoorsPropertyIri) -> bool {
		*self == NUMBER_OF_DOORS_PROPERTY_IRI_HTTP || *self == NUMBER_OF_DOORS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfDoorsPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfDoorsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfDoorsPropertyIri || *other == NUMBER_OF_DOORS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfDoorsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfDoorsPropertyIriOrLabel) -> bool {
		*self == NumberOfDoorsPropertyIri || *self == NUMBER_OF_DOORS_PROPERTY_LABEL
	}
}
