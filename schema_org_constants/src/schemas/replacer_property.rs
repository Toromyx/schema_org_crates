/// <https://schema.org/replacer>
pub const REPLACER_PROPERTY_IRI_HTTP: &str = "http://schema.org/replacer";
/// <https://schema.org/replacer>
pub const REPLACER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/replacer";
/// <https://schema.org/replacer>
pub const REPLACER_PROPERTY_LABEL: &str = "replacer";
pub struct ReplacerPropertyIri;
impl PartialEq<&str> for ReplacerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REPLACER_PROPERTY_IRI_HTTP || *other == REPLACER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReplacerPropertyIri> for &str {
	fn eq(&self, other: &ReplacerPropertyIri) -> bool {
		*self == REPLACER_PROPERTY_IRI_HTTP || *self == REPLACER_PROPERTY_IRI_HTTPS
	}
}
pub struct ReplacerPropertyIriOrLabel;
impl PartialEq<&str> for ReplacerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReplacerPropertyIri || *other == REPLACER_PROPERTY_LABEL
	}
}
impl PartialEq<ReplacerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReplacerPropertyIriOrLabel) -> bool {
		*self == ReplacerPropertyIri || *self == REPLACER_PROPERTY_LABEL
	}
}
