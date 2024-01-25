/// <https://schema.org/yield>
pub const YIELD_PROPERTY_IRI_HTTP: &str = "http://schema.org/yield";
/// <https://schema.org/yield>
pub const YIELD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/yield";
/// <https://schema.org/yield>
pub const YIELD_PROPERTY_LABEL: &str = "yield";
pub struct YieldPropertyIri;
impl PartialEq<&str> for YieldPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == YIELD_PROPERTY_IRI_HTTP || *other == YIELD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<YieldPropertyIri> for &str {
	fn eq(&self, other: &YieldPropertyIri) -> bool {
		*self == YIELD_PROPERTY_IRI_HTTP || *self == YIELD_PROPERTY_IRI_HTTPS
	}
}
pub struct YieldPropertyIriOrLabel;
impl PartialEq<&str> for YieldPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == YieldPropertyIri || *other == YIELD_PROPERTY_LABEL
	}
}
impl PartialEq<YieldPropertyIriOrLabel> for &str {
	fn eq(&self, other: &YieldPropertyIriOrLabel) -> bool {
		*self == YieldPropertyIri || *self == YIELD_PROPERTY_LABEL
	}
}
