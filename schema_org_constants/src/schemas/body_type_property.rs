/// <https://schema.org/bodyType>
pub const BODY_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/bodyType";
/// <https://schema.org/bodyType>
pub const BODY_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/bodyType";
/// <https://schema.org/bodyType>
pub const BODY_TYPE_PROPERTY_LABEL: &str = "bodyType";
pub struct BodyTypePropertyIri;
impl PartialEq<&str> for BodyTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_TYPE_PROPERTY_IRI_HTTP || *other == BODY_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BodyTypePropertyIri> for &str {
	fn eq(&self, other: &BodyTypePropertyIri) -> bool {
		*self == BODY_TYPE_PROPERTY_IRI_HTTP || *self == BODY_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct BodyTypePropertyIriOrLabel;
impl PartialEq<&str> for BodyTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyTypePropertyIri || *other == BODY_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<BodyTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &BodyTypePropertyIriOrLabel) -> bool {
		*self == BodyTypePropertyIri || *self == BODY_TYPE_PROPERTY_LABEL
	}
}
