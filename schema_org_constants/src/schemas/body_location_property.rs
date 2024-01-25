/// <https://schema.org/bodyLocation>
pub const BODY_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/bodyLocation";
/// <https://schema.org/bodyLocation>
pub const BODY_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/bodyLocation";
/// <https://schema.org/bodyLocation>
pub const BODY_LOCATION_PROPERTY_LABEL: &str = "bodyLocation";
pub struct BodyLocationPropertyIri;
impl PartialEq<&str> for BodyLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_LOCATION_PROPERTY_IRI_HTTP || *other == BODY_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BodyLocationPropertyIri> for &str {
	fn eq(&self, other: &BodyLocationPropertyIri) -> bool {
		*self == BODY_LOCATION_PROPERTY_IRI_HTTP || *self == BODY_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct BodyLocationPropertyIriOrLabel;
impl PartialEq<&str> for BodyLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyLocationPropertyIri || *other == BODY_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<BodyLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BodyLocationPropertyIriOrLabel) -> bool {
		*self == BodyLocationPropertyIri || *self == BODY_LOCATION_PROPERTY_LABEL
	}
}
