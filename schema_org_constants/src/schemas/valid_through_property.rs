/// <https://schema.org/validThrough>
pub const VALID_THROUGH_PROPERTY_IRI_HTTP: &str = "http://schema.org/validThrough";
/// <https://schema.org/validThrough>
pub const VALID_THROUGH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/validThrough";
/// <https://schema.org/validThrough>
pub const VALID_THROUGH_PROPERTY_LABEL: &str = "validThrough";
pub struct ValidThroughPropertyIri;
impl PartialEq<&str> for ValidThroughPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VALID_THROUGH_PROPERTY_IRI_HTTP || *other == VALID_THROUGH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ValidThroughPropertyIri> for &str {
	fn eq(&self, other: &ValidThroughPropertyIri) -> bool {
		*self == VALID_THROUGH_PROPERTY_IRI_HTTP || *self == VALID_THROUGH_PROPERTY_IRI_HTTPS
	}
}
pub struct ValidThroughPropertyIriOrLabel;
impl PartialEq<&str> for ValidThroughPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ValidThroughPropertyIri || *other == VALID_THROUGH_PROPERTY_LABEL
	}
}
impl PartialEq<ValidThroughPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ValidThroughPropertyIriOrLabel) -> bool {
		*self == ValidThroughPropertyIri || *self == VALID_THROUGH_PROPERTY_LABEL
	}
}
