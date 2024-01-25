/// <https://schema.org/performer>
pub const PERFORMER_PROPERTY_IRI_HTTP: &str = "http://schema.org/performer";
/// <https://schema.org/performer>
pub const PERFORMER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/performer";
/// <https://schema.org/performer>
pub const PERFORMER_PROPERTY_LABEL: &str = "performer";
pub struct PerformerPropertyIri;
impl PartialEq<&str> for PerformerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERFORMER_PROPERTY_IRI_HTTP || *other == PERFORMER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PerformerPropertyIri> for &str {
	fn eq(&self, other: &PerformerPropertyIri) -> bool {
		*self == PERFORMER_PROPERTY_IRI_HTTP || *self == PERFORMER_PROPERTY_IRI_HTTPS
	}
}
pub struct PerformerPropertyIriOrLabel;
impl PartialEq<&str> for PerformerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PerformerPropertyIri || *other == PERFORMER_PROPERTY_LABEL
	}
}
impl PartialEq<PerformerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PerformerPropertyIriOrLabel) -> bool {
		*self == PerformerPropertyIri || *self == PERFORMER_PROPERTY_LABEL
	}
}
