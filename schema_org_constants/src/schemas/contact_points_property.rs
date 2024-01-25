/// <https://schema.org/contactPoints>
#[deprecated = "This schema is superseded by <https://schema.org/contactPoint>."]
pub const CONTACT_POINTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/contactPoints";
/// <https://schema.org/contactPoints>
#[deprecated = "This schema is superseded by <https://schema.org/contactPoint>."]
pub const CONTACT_POINTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/contactPoints";
/// <https://schema.org/contactPoints>
#[deprecated = "This schema is superseded by <https://schema.org/contactPoint>."]
pub const CONTACT_POINTS_PROPERTY_LABEL: &str = "contactPoints";
pub struct ContactPointsPropertyIri;
impl PartialEq<&str> for ContactPointsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTACT_POINTS_PROPERTY_IRI_HTTP || *other == CONTACT_POINTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContactPointsPropertyIri> for &str {
	fn eq(&self, other: &ContactPointsPropertyIri) -> bool {
		*self == CONTACT_POINTS_PROPERTY_IRI_HTTP || *self == CONTACT_POINTS_PROPERTY_IRI_HTTPS
	}
}
pub struct ContactPointsPropertyIriOrLabel;
impl PartialEq<&str> for ContactPointsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContactPointsPropertyIri || *other == CONTACT_POINTS_PROPERTY_LABEL
	}
}
impl PartialEq<ContactPointsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContactPointsPropertyIriOrLabel) -> bool {
		*self == ContactPointsPropertyIri || *self == CONTACT_POINTS_PROPERTY_LABEL
	}
}
