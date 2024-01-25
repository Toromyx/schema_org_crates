/// <https://schema.org/ContactPoint>
pub const CONTACT_POINT_IRI_HTTP: &str = "http://schema.org/ContactPoint";
/// <https://schema.org/ContactPoint>
pub const CONTACT_POINT_IRI_HTTPS: &str = "https://schema.org/ContactPoint";
/// <https://schema.org/ContactPoint>
pub const CONTACT_POINT_LABEL: &str = "ContactPoint";
pub struct ContactPointIri;
impl PartialEq<&str> for ContactPointIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTACT_POINT_IRI_HTTP || *other == CONTACT_POINT_IRI_HTTPS
	}
}
impl PartialEq<ContactPointIri> for &str {
	fn eq(&self, other: &ContactPointIri) -> bool {
		*self == CONTACT_POINT_IRI_HTTP || *self == CONTACT_POINT_IRI_HTTPS
	}
}
pub struct ContactPointIriOrLabel;
impl PartialEq<&str> for ContactPointIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContactPointIri || *other == CONTACT_POINT_LABEL
	}
}
impl PartialEq<ContactPointIriOrLabel> for &str {
	fn eq(&self, other: &ContactPointIriOrLabel) -> bool {
		*self == ContactPointIri || *self == CONTACT_POINT_LABEL
	}
}
