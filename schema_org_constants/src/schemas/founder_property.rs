/// <https://schema.org/founder>
pub const FOUNDER_PROPERTY_IRI_HTTP: &str = "http://schema.org/founder";
/// <https://schema.org/founder>
pub const FOUNDER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/founder";
/// <https://schema.org/founder>
pub const FOUNDER_PROPERTY_LABEL: &str = "founder";
pub struct FounderPropertyIri;
impl PartialEq<&str> for FounderPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOUNDER_PROPERTY_IRI_HTTP || *other == FOUNDER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FounderPropertyIri> for &str {
	fn eq(&self, other: &FounderPropertyIri) -> bool {
		*self == FOUNDER_PROPERTY_IRI_HTTP || *self == FOUNDER_PROPERTY_IRI_HTTPS
	}
}
pub struct FounderPropertyIriOrLabel;
impl PartialEq<&str> for FounderPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FounderPropertyIri || *other == FOUNDER_PROPERTY_LABEL
	}
}
impl PartialEq<FounderPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FounderPropertyIriOrLabel) -> bool {
		*self == FounderPropertyIri || *self == FOUNDER_PROPERTY_LABEL
	}
}
