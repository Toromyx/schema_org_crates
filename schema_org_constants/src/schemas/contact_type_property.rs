/// <https://schema.org/contactType>
pub const CONTACT_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/contactType";
/// <https://schema.org/contactType>
pub const CONTACT_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/contactType";
/// <https://schema.org/contactType>
pub const CONTACT_TYPE_PROPERTY_LABEL: &str = "contactType";
pub struct ContactTypePropertyIri;
impl PartialEq<&str> for ContactTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTACT_TYPE_PROPERTY_IRI_HTTP || *other == CONTACT_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContactTypePropertyIri> for &str {
	fn eq(&self, other: &ContactTypePropertyIri) -> bool {
		*self == CONTACT_TYPE_PROPERTY_IRI_HTTP || *self == CONTACT_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct ContactTypePropertyIriOrLabel;
impl PartialEq<&str> for ContactTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContactTypePropertyIri || *other == CONTACT_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<ContactTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContactTypePropertyIriOrLabel) -> bool {
		*self == ContactTypePropertyIri || *self == CONTACT_TYPE_PROPERTY_LABEL
	}
}
