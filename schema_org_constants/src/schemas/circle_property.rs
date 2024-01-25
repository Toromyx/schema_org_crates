/// <https://schema.org/circle>
pub const CIRCLE_PROPERTY_IRI_HTTP: &str = "http://schema.org/circle";
/// <https://schema.org/circle>
pub const CIRCLE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/circle";
/// <https://schema.org/circle>
pub const CIRCLE_PROPERTY_LABEL: &str = "circle";
pub struct CirclePropertyIri;
impl PartialEq<&str> for CirclePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CIRCLE_PROPERTY_IRI_HTTP || *other == CIRCLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CirclePropertyIri> for &str {
	fn eq(&self, other: &CirclePropertyIri) -> bool {
		*self == CIRCLE_PROPERTY_IRI_HTTP || *self == CIRCLE_PROPERTY_IRI_HTTPS
	}
}
pub struct CirclePropertyIriOrLabel;
impl PartialEq<&str> for CirclePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CirclePropertyIri || *other == CIRCLE_PROPERTY_LABEL
	}
}
impl PartialEq<CirclePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CirclePropertyIriOrLabel) -> bool {
		*self == CirclePropertyIri || *self == CIRCLE_PROPERTY_LABEL
	}
}
