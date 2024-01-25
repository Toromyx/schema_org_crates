/// <https://schema.org/confirmationNumber>
pub const CONFIRMATION_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/confirmationNumber";
/// <https://schema.org/confirmationNumber>
pub const CONFIRMATION_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/confirmationNumber";
/// <https://schema.org/confirmationNumber>
pub const CONFIRMATION_NUMBER_PROPERTY_LABEL: &str = "confirmationNumber";
pub struct ConfirmationNumberPropertyIri;
impl PartialEq<&str> for ConfirmationNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONFIRMATION_NUMBER_PROPERTY_IRI_HTTP
			|| *other == CONFIRMATION_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ConfirmationNumberPropertyIri> for &str {
	fn eq(&self, other: &ConfirmationNumberPropertyIri) -> bool {
		*self == CONFIRMATION_NUMBER_PROPERTY_IRI_HTTP
			|| *self == CONFIRMATION_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct ConfirmationNumberPropertyIriOrLabel;
impl PartialEq<&str> for ConfirmationNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ConfirmationNumberPropertyIri || *other == CONFIRMATION_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<ConfirmationNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ConfirmationNumberPropertyIriOrLabel) -> bool {
		*self == ConfirmationNumberPropertyIri || *self == CONFIRMATION_NUMBER_PROPERTY_LABEL
	}
}
