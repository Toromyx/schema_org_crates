/// <https://schema.org/ContactPage>
pub const CONTACT_PAGE_IRI_HTTP: &str = "http://schema.org/ContactPage";
/// <https://schema.org/ContactPage>
pub const CONTACT_PAGE_IRI_HTTPS: &str = "https://schema.org/ContactPage";
/// <https://schema.org/ContactPage>
pub const CONTACT_PAGE_LABEL: &str = "ContactPage";
pub struct ContactPageIri;
impl PartialEq<&str> for ContactPageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTACT_PAGE_IRI_HTTP || *other == CONTACT_PAGE_IRI_HTTPS
	}
}
impl PartialEq<ContactPageIri> for &str {
	fn eq(&self, other: &ContactPageIri) -> bool {
		*self == CONTACT_PAGE_IRI_HTTP || *self == CONTACT_PAGE_IRI_HTTPS
	}
}
pub struct ContactPageIriOrLabel;
impl PartialEq<&str> for ContactPageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContactPageIri || *other == CONTACT_PAGE_LABEL
	}
}
impl PartialEq<ContactPageIriOrLabel> for &str {
	fn eq(&self, other: &ContactPageIriOrLabel) -> bool {
		*self == ContactPageIri || *self == CONTACT_PAGE_LABEL
	}
}
