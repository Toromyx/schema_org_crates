/// <https://schema.org/email>
pub const EMAIL_PROPERTY_IRI_HTTP: &str = "http://schema.org/email";
/// <https://schema.org/email>
pub const EMAIL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/email";
/// <https://schema.org/email>
pub const EMAIL_PROPERTY_LABEL: &str = "email";
pub struct EmailPropertyIri;
impl PartialEq<&str> for EmailPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMAIL_PROPERTY_IRI_HTTP || *other == EMAIL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EmailPropertyIri> for &str {
	fn eq(&self, other: &EmailPropertyIri) -> bool {
		*self == EMAIL_PROPERTY_IRI_HTTP || *self == EMAIL_PROPERTY_IRI_HTTPS
	}
}
pub struct EmailPropertyIriOrLabel;
impl PartialEq<&str> for EmailPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmailPropertyIri || *other == EMAIL_PROPERTY_LABEL
	}
}
impl PartialEq<EmailPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EmailPropertyIriOrLabel) -> bool {
		*self == EmailPropertyIri || *self == EMAIL_PROPERTY_LABEL
	}
}
