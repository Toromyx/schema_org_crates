/// <https://schema.org/numberOfPreviousOwners>
pub const NUMBER_OF_PREVIOUS_OWNERS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/numberOfPreviousOwners";
/// <https://schema.org/numberOfPreviousOwners>
pub const NUMBER_OF_PREVIOUS_OWNERS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/numberOfPreviousOwners";
/// <https://schema.org/numberOfPreviousOwners>
pub const NUMBER_OF_PREVIOUS_OWNERS_PROPERTY_LABEL: &str = "numberOfPreviousOwners";
pub struct NumberOfPreviousOwnersPropertyIri;
impl PartialEq<&str> for NumberOfPreviousOwnersPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_PREVIOUS_OWNERS_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_PREVIOUS_OWNERS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfPreviousOwnersPropertyIri> for &str {
	fn eq(&self, other: &NumberOfPreviousOwnersPropertyIri) -> bool {
		*self == NUMBER_OF_PREVIOUS_OWNERS_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_PREVIOUS_OWNERS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfPreviousOwnersPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfPreviousOwnersPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfPreviousOwnersPropertyIri
			|| *other == NUMBER_OF_PREVIOUS_OWNERS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfPreviousOwnersPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfPreviousOwnersPropertyIriOrLabel) -> bool {
		*self == NumberOfPreviousOwnersPropertyIri
			|| *self == NUMBER_OF_PREVIOUS_OWNERS_PROPERTY_LABEL
	}
}
