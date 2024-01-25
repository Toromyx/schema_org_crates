/// <https://schema.org/contactOption>
pub const CONTACT_OPTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/contactOption";
/// <https://schema.org/contactOption>
pub const CONTACT_OPTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/contactOption";
/// <https://schema.org/contactOption>
pub const CONTACT_OPTION_PROPERTY_LABEL: &str = "contactOption";
pub struct ContactOptionPropertyIri;
impl PartialEq<&str> for ContactOptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTACT_OPTION_PROPERTY_IRI_HTTP || *other == CONTACT_OPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContactOptionPropertyIri> for &str {
	fn eq(&self, other: &ContactOptionPropertyIri) -> bool {
		*self == CONTACT_OPTION_PROPERTY_IRI_HTTP || *self == CONTACT_OPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct ContactOptionPropertyIriOrLabel;
impl PartialEq<&str> for ContactOptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContactOptionPropertyIri || *other == CONTACT_OPTION_PROPERTY_LABEL
	}
}
impl PartialEq<ContactOptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContactOptionPropertyIriOrLabel) -> bool {
		*self == ContactOptionPropertyIri || *self == CONTACT_OPTION_PROPERTY_LABEL
	}
}
