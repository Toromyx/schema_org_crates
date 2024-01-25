/// <https://schema.org/minValue>
pub const MIN_VALUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/minValue";
/// <https://schema.org/minValue>
pub const MIN_VALUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/minValue";
/// <https://schema.org/minValue>
pub const MIN_VALUE_PROPERTY_LABEL: &str = "minValue";
pub struct MinValuePropertyIri;
impl PartialEq<&str> for MinValuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MIN_VALUE_PROPERTY_IRI_HTTP || *other == MIN_VALUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MinValuePropertyIri> for &str {
	fn eq(&self, other: &MinValuePropertyIri) -> bool {
		*self == MIN_VALUE_PROPERTY_IRI_HTTP || *self == MIN_VALUE_PROPERTY_IRI_HTTPS
	}
}
pub struct MinValuePropertyIriOrLabel;
impl PartialEq<&str> for MinValuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MinValuePropertyIri || *other == MIN_VALUE_PROPERTY_LABEL
	}
}
impl PartialEq<MinValuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &MinValuePropertyIriOrLabel) -> bool {
		*self == MinValuePropertyIri || *self == MIN_VALUE_PROPERTY_LABEL
	}
}
