/// <https://schema.org/Artery>
pub const ARTERY_IRI_HTTP: &str = "http://schema.org/Artery";
/// <https://schema.org/Artery>
pub const ARTERY_IRI_HTTPS: &str = "https://schema.org/Artery";
/// <https://schema.org/Artery>
pub const ARTERY_LABEL: &str = "Artery";
pub struct ArteryIri;
impl PartialEq<&str> for ArteryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARTERY_IRI_HTTP || *other == ARTERY_IRI_HTTPS
	}
}
impl PartialEq<ArteryIri> for &str {
	fn eq(&self, other: &ArteryIri) -> bool {
		*self == ARTERY_IRI_HTTP || *self == ARTERY_IRI_HTTPS
	}
}
pub struct ArteryIriOrLabel;
impl PartialEq<&str> for ArteryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArteryIri || *other == ARTERY_LABEL
	}
}
impl PartialEq<ArteryIriOrLabel> for &str {
	fn eq(&self, other: &ArteryIriOrLabel) -> bool {
		*self == ArteryIri || *self == ARTERY_LABEL
	}
}
