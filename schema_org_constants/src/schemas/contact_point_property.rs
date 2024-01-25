/// <https://schema.org/contactPoint>
pub const CONTACT_POINT_PROPERTY_IRI_HTTP: &str = "http://schema.org/contactPoint";
/// <https://schema.org/contactPoint>
pub const CONTACT_POINT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/contactPoint";
/// <https://schema.org/contactPoint>
pub const CONTACT_POINT_PROPERTY_LABEL: &str = "contactPoint";
pub struct ContactPointPropertyIri;
impl PartialEq<&str> for ContactPointPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTACT_POINT_PROPERTY_IRI_HTTP || *other == CONTACT_POINT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContactPointPropertyIri> for &str {
	fn eq(&self, other: &ContactPointPropertyIri) -> bool {
		*self == CONTACT_POINT_PROPERTY_IRI_HTTP || *self == CONTACT_POINT_PROPERTY_IRI_HTTPS
	}
}
pub struct ContactPointPropertyIriOrLabel;
impl PartialEq<&str> for ContactPointPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContactPointPropertyIri || *other == CONTACT_POINT_PROPERTY_LABEL
	}
}
impl PartialEq<ContactPointPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContactPointPropertyIriOrLabel) -> bool {
		*self == ContactPointPropertyIri || *self == CONTACT_POINT_PROPERTY_LABEL
	}
}
