/// <https://schema.org/maxValue>
pub const MAX_VALUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/maxValue";
/// <https://schema.org/maxValue>
pub const MAX_VALUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/maxValue";
/// <https://schema.org/maxValue>
pub const MAX_VALUE_PROPERTY_LABEL: &str = "maxValue";
pub struct MaxValuePropertyIri;
impl PartialEq<&str> for MaxValuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAX_VALUE_PROPERTY_IRI_HTTP || *other == MAX_VALUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MaxValuePropertyIri> for &str {
	fn eq(&self, other: &MaxValuePropertyIri) -> bool {
		*self == MAX_VALUE_PROPERTY_IRI_HTTP || *self == MAX_VALUE_PROPERTY_IRI_HTTPS
	}
}
pub struct MaxValuePropertyIriOrLabel;
impl PartialEq<&str> for MaxValuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MaxValuePropertyIri || *other == MAX_VALUE_PROPERTY_LABEL
	}
}
impl PartialEq<MaxValuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &MaxValuePropertyIriOrLabel) -> bool {
		*self == MaxValuePropertyIri || *self == MAX_VALUE_PROPERTY_LABEL
	}
}
