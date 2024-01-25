/// <https://schema.org/additionalNumberOfGuests>
pub const ADDITIONAL_NUMBER_OF_GUESTS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/additionalNumberOfGuests";
/// <https://schema.org/additionalNumberOfGuests>
pub const ADDITIONAL_NUMBER_OF_GUESTS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/additionalNumberOfGuests";
/// <https://schema.org/additionalNumberOfGuests>
pub const ADDITIONAL_NUMBER_OF_GUESTS_PROPERTY_LABEL: &str = "additionalNumberOfGuests";
pub struct AdditionalNumberOfGuestsPropertyIri;
impl PartialEq<&str> for AdditionalNumberOfGuestsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADDITIONAL_NUMBER_OF_GUESTS_PROPERTY_IRI_HTTP
			|| *other == ADDITIONAL_NUMBER_OF_GUESTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AdditionalNumberOfGuestsPropertyIri> for &str {
	fn eq(&self, other: &AdditionalNumberOfGuestsPropertyIri) -> bool {
		*self == ADDITIONAL_NUMBER_OF_GUESTS_PROPERTY_IRI_HTTP
			|| *self == ADDITIONAL_NUMBER_OF_GUESTS_PROPERTY_IRI_HTTPS
	}
}
pub struct AdditionalNumberOfGuestsPropertyIriOrLabel;
impl PartialEq<&str> for AdditionalNumberOfGuestsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AdditionalNumberOfGuestsPropertyIri
			|| *other == ADDITIONAL_NUMBER_OF_GUESTS_PROPERTY_LABEL
	}
}
impl PartialEq<AdditionalNumberOfGuestsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AdditionalNumberOfGuestsPropertyIriOrLabel) -> bool {
		*self == AdditionalNumberOfGuestsPropertyIri
			|| *self == ADDITIONAL_NUMBER_OF_GUESTS_PROPERTY_LABEL
	}
}
