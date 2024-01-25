/// <https://schema.org/valuePattern>
pub const VALUE_PATTERN_PROPERTY_IRI_HTTP: &str = "http://schema.org/valuePattern";
/// <https://schema.org/valuePattern>
pub const VALUE_PATTERN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/valuePattern";
/// <https://schema.org/valuePattern>
pub const VALUE_PATTERN_PROPERTY_LABEL: &str = "valuePattern";
pub struct ValuePatternPropertyIri;
impl PartialEq<&str> for ValuePatternPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VALUE_PATTERN_PROPERTY_IRI_HTTP || *other == VALUE_PATTERN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ValuePatternPropertyIri> for &str {
	fn eq(&self, other: &ValuePatternPropertyIri) -> bool {
		*self == VALUE_PATTERN_PROPERTY_IRI_HTTP || *self == VALUE_PATTERN_PROPERTY_IRI_HTTPS
	}
}
pub struct ValuePatternPropertyIriOrLabel;
impl PartialEq<&str> for ValuePatternPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ValuePatternPropertyIri || *other == VALUE_PATTERN_PROPERTY_LABEL
	}
}
impl PartialEq<ValuePatternPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ValuePatternPropertyIriOrLabel) -> bool {
		*self == ValuePatternPropertyIri || *self == VALUE_PATTERN_PROPERTY_LABEL
	}
}
