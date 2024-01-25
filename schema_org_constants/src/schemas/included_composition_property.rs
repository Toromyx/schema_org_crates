/// <https://schema.org/includedComposition>
pub const INCLUDED_COMPOSITION_PROPERTY_IRI_HTTP: &str = "http://schema.org/includedComposition";
/// <https://schema.org/includedComposition>
pub const INCLUDED_COMPOSITION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/includedComposition";
/// <https://schema.org/includedComposition>
pub const INCLUDED_COMPOSITION_PROPERTY_LABEL: &str = "includedComposition";
pub struct IncludedCompositionPropertyIri;
impl PartialEq<&str> for IncludedCompositionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INCLUDED_COMPOSITION_PROPERTY_IRI_HTTP
			|| *other == INCLUDED_COMPOSITION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IncludedCompositionPropertyIri> for &str {
	fn eq(&self, other: &IncludedCompositionPropertyIri) -> bool {
		*self == INCLUDED_COMPOSITION_PROPERTY_IRI_HTTP
			|| *self == INCLUDED_COMPOSITION_PROPERTY_IRI_HTTPS
	}
}
pub struct IncludedCompositionPropertyIriOrLabel;
impl PartialEq<&str> for IncludedCompositionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IncludedCompositionPropertyIri || *other == INCLUDED_COMPOSITION_PROPERTY_LABEL
	}
}
impl PartialEq<IncludedCompositionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IncludedCompositionPropertyIriOrLabel) -> bool {
		*self == IncludedCompositionPropertyIri || *self == INCLUDED_COMPOSITION_PROPERTY_LABEL
	}
}
