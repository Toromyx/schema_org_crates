/// <https://schema.org/Volcano>
pub const VOLCANO_IRI_HTTP: &str = "http://schema.org/Volcano";
/// <https://schema.org/Volcano>
pub const VOLCANO_IRI_HTTPS: &str = "https://schema.org/Volcano";
/// <https://schema.org/Volcano>
pub const VOLCANO_LABEL: &str = "Volcano";
pub struct VolcanoIri;
impl PartialEq<&str> for VolcanoIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VOLCANO_IRI_HTTP || *other == VOLCANO_IRI_HTTPS
	}
}
impl PartialEq<VolcanoIri> for &str {
	fn eq(&self, other: &VolcanoIri) -> bool {
		*self == VOLCANO_IRI_HTTP || *self == VOLCANO_IRI_HTTPS
	}
}
pub struct VolcanoIriOrLabel;
impl PartialEq<&str> for VolcanoIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VolcanoIri || *other == VOLCANO_LABEL
	}
}
impl PartialEq<VolcanoIriOrLabel> for &str {
	fn eq(&self, other: &VolcanoIriOrLabel) -> bool {
		*self == VolcanoIri || *self == VOLCANO_LABEL
	}
}
