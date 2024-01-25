/// <https://schema.org/endorsee>
pub const ENDORSEE_PROPERTY_IRI_HTTP: &str = "http://schema.org/endorsee";
/// <https://schema.org/endorsee>
pub const ENDORSEE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/endorsee";
/// <https://schema.org/endorsee>
pub const ENDORSEE_PROPERTY_LABEL: &str = "endorsee";
pub struct EndorseePropertyIri;
impl PartialEq<&str> for EndorseePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENDORSEE_PROPERTY_IRI_HTTP || *other == ENDORSEE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EndorseePropertyIri> for &str {
	fn eq(&self, other: &EndorseePropertyIri) -> bool {
		*self == ENDORSEE_PROPERTY_IRI_HTTP || *self == ENDORSEE_PROPERTY_IRI_HTTPS
	}
}
pub struct EndorseePropertyIriOrLabel;
impl PartialEq<&str> for EndorseePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EndorseePropertyIri || *other == ENDORSEE_PROPERTY_LABEL
	}
}
impl PartialEq<EndorseePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EndorseePropertyIriOrLabel) -> bool {
		*self == EndorseePropertyIri || *self == ENDORSEE_PROPERTY_LABEL
	}
}
