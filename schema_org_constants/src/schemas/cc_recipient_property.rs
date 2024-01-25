/// <https://schema.org/ccRecipient>
pub const CC_RECIPIENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/ccRecipient";
/// <https://schema.org/ccRecipient>
pub const CC_RECIPIENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/ccRecipient";
/// <https://schema.org/ccRecipient>
pub const CC_RECIPIENT_PROPERTY_LABEL: &str = "ccRecipient";
pub struct CcRecipientPropertyIri;
impl PartialEq<&str> for CcRecipientPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CC_RECIPIENT_PROPERTY_IRI_HTTP || *other == CC_RECIPIENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CcRecipientPropertyIri> for &str {
	fn eq(&self, other: &CcRecipientPropertyIri) -> bool {
		*self == CC_RECIPIENT_PROPERTY_IRI_HTTP || *self == CC_RECIPIENT_PROPERTY_IRI_HTTPS
	}
}
pub struct CcRecipientPropertyIriOrLabel;
impl PartialEq<&str> for CcRecipientPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CcRecipientPropertyIri || *other == CC_RECIPIENT_PROPERTY_LABEL
	}
}
impl PartialEq<CcRecipientPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CcRecipientPropertyIriOrLabel) -> bool {
		*self == CcRecipientPropertyIri || *self == CC_RECIPIENT_PROPERTY_LABEL
	}
}
