/// <https://schema.org/ContactPointOption>
pub const CONTACT_POINT_OPTION_IRI_HTTP: &str = "http://schema.org/ContactPointOption";
/// <https://schema.org/ContactPointOption>
pub const CONTACT_POINT_OPTION_IRI_HTTPS: &str = "https://schema.org/ContactPointOption";
/// <https://schema.org/ContactPointOption>
pub const CONTACT_POINT_OPTION_LABEL: &str = "ContactPointOption";
pub struct ContactPointOptionIri;
impl PartialEq<&str> for ContactPointOptionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTACT_POINT_OPTION_IRI_HTTP || *other == CONTACT_POINT_OPTION_IRI_HTTPS
	}
}
impl PartialEq<ContactPointOptionIri> for &str {
	fn eq(&self, other: &ContactPointOptionIri) -> bool {
		*self == CONTACT_POINT_OPTION_IRI_HTTP || *self == CONTACT_POINT_OPTION_IRI_HTTPS
	}
}
pub struct ContactPointOptionIriOrLabel;
impl PartialEq<&str> for ContactPointOptionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContactPointOptionIri || *other == CONTACT_POINT_OPTION_LABEL
	}
}
impl PartialEq<ContactPointOptionIriOrLabel> for &str {
	fn eq(&self, other: &ContactPointOptionIriOrLabel) -> bool {
		*self == ContactPointOptionIri || *self == CONTACT_POINT_OPTION_LABEL
	}
}
