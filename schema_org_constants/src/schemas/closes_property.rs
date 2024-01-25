/// <https://schema.org/closes>
pub const CLOSES_PROPERTY_IRI_HTTP: &str = "http://schema.org/closes";
/// <https://schema.org/closes>
pub const CLOSES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/closes";
/// <https://schema.org/closes>
pub const CLOSES_PROPERTY_LABEL: &str = "closes";
pub struct ClosesPropertyIri;
impl PartialEq<&str> for ClosesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CLOSES_PROPERTY_IRI_HTTP || *other == CLOSES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ClosesPropertyIri> for &str {
	fn eq(&self, other: &ClosesPropertyIri) -> bool {
		*self == CLOSES_PROPERTY_IRI_HTTP || *self == CLOSES_PROPERTY_IRI_HTTPS
	}
}
pub struct ClosesPropertyIriOrLabel;
impl PartialEq<&str> for ClosesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ClosesPropertyIri || *other == CLOSES_PROPERTY_LABEL
	}
}
impl PartialEq<ClosesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ClosesPropertyIriOrLabel) -> bool {
		*self == ClosesPropertyIri || *self == CLOSES_PROPERTY_LABEL
	}
}
