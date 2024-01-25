/// <https://schema.org/Synagogue>
pub const SYNAGOGUE_IRI_HTTP: &str = "http://schema.org/Synagogue";
/// <https://schema.org/Synagogue>
pub const SYNAGOGUE_IRI_HTTPS: &str = "https://schema.org/Synagogue";
/// <https://schema.org/Synagogue>
pub const SYNAGOGUE_LABEL: &str = "Synagogue";
pub struct SynagogueIri;
impl PartialEq<&str> for SynagogueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SYNAGOGUE_IRI_HTTP || *other == SYNAGOGUE_IRI_HTTPS
	}
}
impl PartialEq<SynagogueIri> for &str {
	fn eq(&self, other: &SynagogueIri) -> bool {
		*self == SYNAGOGUE_IRI_HTTP || *self == SYNAGOGUE_IRI_HTTPS
	}
}
pub struct SynagogueIriOrLabel;
impl PartialEq<&str> for SynagogueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SynagogueIri || *other == SYNAGOGUE_LABEL
	}
}
impl PartialEq<SynagogueIriOrLabel> for &str {
	fn eq(&self, other: &SynagogueIriOrLabel) -> bool {
		*self == SynagogueIri || *self == SYNAGOGUE_LABEL
	}
}
