/// <https://schema.org/numberOfForwardGears>
pub const NUMBER_OF_FORWARD_GEARS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/numberOfForwardGears";
/// <https://schema.org/numberOfForwardGears>
pub const NUMBER_OF_FORWARD_GEARS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/numberOfForwardGears";
/// <https://schema.org/numberOfForwardGears>
pub const NUMBER_OF_FORWARD_GEARS_PROPERTY_LABEL: &str = "numberOfForwardGears";
pub struct NumberOfForwardGearsPropertyIri;
impl PartialEq<&str> for NumberOfForwardGearsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_FORWARD_GEARS_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_FORWARD_GEARS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfForwardGearsPropertyIri> for &str {
	fn eq(&self, other: &NumberOfForwardGearsPropertyIri) -> bool {
		*self == NUMBER_OF_FORWARD_GEARS_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_FORWARD_GEARS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfForwardGearsPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfForwardGearsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfForwardGearsPropertyIri
			|| *other == NUMBER_OF_FORWARD_GEARS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfForwardGearsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfForwardGearsPropertyIriOrLabel) -> bool {
		*self == NumberOfForwardGearsPropertyIri || *self == NUMBER_OF_FORWARD_GEARS_PROPERTY_LABEL
	}
}
