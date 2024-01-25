/// <https://schema.org/recipient>
pub const RECIPIENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/recipient";
/// <https://schema.org/recipient>
pub const RECIPIENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recipient";
/// <https://schema.org/recipient>
pub const RECIPIENT_PROPERTY_LABEL: &str = "recipient";
pub struct RecipientPropertyIri;
impl PartialEq<&str> for RecipientPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECIPIENT_PROPERTY_IRI_HTTP || *other == RECIPIENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecipientPropertyIri> for &str {
	fn eq(&self, other: &RecipientPropertyIri) -> bool {
		*self == RECIPIENT_PROPERTY_IRI_HTTP || *self == RECIPIENT_PROPERTY_IRI_HTTPS
	}
}
pub struct RecipientPropertyIriOrLabel;
impl PartialEq<&str> for RecipientPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecipientPropertyIri || *other == RECIPIENT_PROPERTY_LABEL
	}
}
impl PartialEq<RecipientPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecipientPropertyIriOrLabel) -> bool {
		*self == RecipientPropertyIri || *self == RECIPIENT_PROPERTY_LABEL
	}
}
