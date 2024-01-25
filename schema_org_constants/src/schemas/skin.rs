/// <https://schema.org/Skin>
pub const SKIN_IRI_HTTP: &str = "http://schema.org/Skin";
/// <https://schema.org/Skin>
pub const SKIN_IRI_HTTPS: &str = "https://schema.org/Skin";
/// <https://schema.org/Skin>
pub const SKIN_LABEL: &str = "Skin";
pub struct SkinIri;
impl PartialEq<&str> for SkinIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SKIN_IRI_HTTP || *other == SKIN_IRI_HTTPS
	}
}
impl PartialEq<SkinIri> for &str {
	fn eq(&self, other: &SkinIri) -> bool {
		*self == SKIN_IRI_HTTP || *self == SKIN_IRI_HTTPS
	}
}
pub struct SkinIriOrLabel;
impl PartialEq<&str> for SkinIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SkinIri || *other == SKIN_LABEL
	}
}
impl PartialEq<SkinIriOrLabel> for &str {
	fn eq(&self, other: &SkinIriOrLabel) -> bool {
		*self == SkinIri || *self == SKIN_LABEL
	}
}
