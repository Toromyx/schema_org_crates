/// <https://schema.org/identifier>
pub const IDENTIFIER_PROPERTY_IRI_HTTP: &str = "http://schema.org/identifier";
/// <https://schema.org/identifier>
pub const IDENTIFIER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/identifier";
/// <https://schema.org/identifier>
pub const IDENTIFIER_PROPERTY_LABEL: &str = "identifier";
pub struct IdentifierPropertyIri;
impl PartialEq<&str> for IdentifierPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IDENTIFIER_PROPERTY_IRI_HTTP || *other == IDENTIFIER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IdentifierPropertyIri> for &str {
	fn eq(&self, other: &IdentifierPropertyIri) -> bool {
		*self == IDENTIFIER_PROPERTY_IRI_HTTP || *self == IDENTIFIER_PROPERTY_IRI_HTTPS
	}
}
pub struct IdentifierPropertyIriOrLabel;
impl PartialEq<&str> for IdentifierPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IdentifierPropertyIri || *other == IDENTIFIER_PROPERTY_LABEL
	}
}
impl PartialEq<IdentifierPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IdentifierPropertyIriOrLabel) -> bool {
		*self == IdentifierPropertyIri || *self == IDENTIFIER_PROPERTY_LABEL
	}
}
