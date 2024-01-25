/// <https://schema.org/discusses>
pub const DISCUSSES_PROPERTY_IRI_HTTP: &str = "http://schema.org/discusses";
/// <https://schema.org/discusses>
pub const DISCUSSES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/discusses";
/// <https://schema.org/discusses>
pub const DISCUSSES_PROPERTY_LABEL: &str = "discusses";
pub struct DiscussesPropertyIri;
impl PartialEq<&str> for DiscussesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISCUSSES_PROPERTY_IRI_HTTP || *other == DISCUSSES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DiscussesPropertyIri> for &str {
	fn eq(&self, other: &DiscussesPropertyIri) -> bool {
		*self == DISCUSSES_PROPERTY_IRI_HTTP || *self == DISCUSSES_PROPERTY_IRI_HTTPS
	}
}
pub struct DiscussesPropertyIriOrLabel;
impl PartialEq<&str> for DiscussesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiscussesPropertyIri || *other == DISCUSSES_PROPERTY_LABEL
	}
}
impl PartialEq<DiscussesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DiscussesPropertyIriOrLabel) -> bool {
		*self == DiscussesPropertyIri || *self == DISCUSSES_PROPERTY_LABEL
	}
}
