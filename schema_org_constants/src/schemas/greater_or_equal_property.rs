/// <https://schema.org/greaterOrEqual>
pub const GREATER_OR_EQUAL_PROPERTY_IRI_HTTP: &str = "http://schema.org/greaterOrEqual";
/// <https://schema.org/greaterOrEqual>
pub const GREATER_OR_EQUAL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/greaterOrEqual";
/// <https://schema.org/greaterOrEqual>
pub const GREATER_OR_EQUAL_PROPERTY_LABEL: &str = "greaterOrEqual";
pub struct GreaterOrEqualPropertyIri;
impl PartialEq<&str> for GreaterOrEqualPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GREATER_OR_EQUAL_PROPERTY_IRI_HTTP
			|| *other == GREATER_OR_EQUAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GreaterOrEqualPropertyIri> for &str {
	fn eq(&self, other: &GreaterOrEqualPropertyIri) -> bool {
		*self == GREATER_OR_EQUAL_PROPERTY_IRI_HTTP || *self == GREATER_OR_EQUAL_PROPERTY_IRI_HTTPS
	}
}
pub struct GreaterOrEqualPropertyIriOrLabel;
impl PartialEq<&str> for GreaterOrEqualPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GreaterOrEqualPropertyIri || *other == GREATER_OR_EQUAL_PROPERTY_LABEL
	}
}
impl PartialEq<GreaterOrEqualPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GreaterOrEqualPropertyIriOrLabel) -> bool {
		*self == GreaterOrEqualPropertyIri || *self == GREATER_OR_EQUAL_PROPERTY_LABEL
	}
}
