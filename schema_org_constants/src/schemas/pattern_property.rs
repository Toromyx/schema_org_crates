/// <https://schema.org/pattern>
pub const PATTERN_PROPERTY_IRI_HTTP: &str = "http://schema.org/pattern";
/// <https://schema.org/pattern>
pub const PATTERN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/pattern";
/// <https://schema.org/pattern>
pub const PATTERN_PROPERTY_LABEL: &str = "pattern";
pub struct PatternPropertyIri;
impl PartialEq<&str> for PatternPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PATTERN_PROPERTY_IRI_HTTP || *other == PATTERN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PatternPropertyIri> for &str {
	fn eq(&self, other: &PatternPropertyIri) -> bool {
		*self == PATTERN_PROPERTY_IRI_HTTP || *self == PATTERN_PROPERTY_IRI_HTTPS
	}
}
pub struct PatternPropertyIriOrLabel;
impl PartialEq<&str> for PatternPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PatternPropertyIri || *other == PATTERN_PROPERTY_LABEL
	}
}
impl PartialEq<PatternPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PatternPropertyIriOrLabel) -> bool {
		*self == PatternPropertyIri || *self == PATTERN_PROPERTY_LABEL
	}
}
