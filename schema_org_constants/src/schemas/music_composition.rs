/// <https://schema.org/MusicComposition>
pub const MUSIC_COMPOSITION_IRI_HTTP: &str = "http://schema.org/MusicComposition";
/// <https://schema.org/MusicComposition>
pub const MUSIC_COMPOSITION_IRI_HTTPS: &str = "https://schema.org/MusicComposition";
/// <https://schema.org/MusicComposition>
pub const MUSIC_COMPOSITION_LABEL: &str = "MusicComposition";
pub struct MusicCompositionIri;
impl PartialEq<&str> for MusicCompositionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_COMPOSITION_IRI_HTTP || *other == MUSIC_COMPOSITION_IRI_HTTPS
	}
}
impl PartialEq<MusicCompositionIri> for &str {
	fn eq(&self, other: &MusicCompositionIri) -> bool {
		*self == MUSIC_COMPOSITION_IRI_HTTP || *self == MUSIC_COMPOSITION_IRI_HTTPS
	}
}
pub struct MusicCompositionIriOrLabel;
impl PartialEq<&str> for MusicCompositionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicCompositionIri || *other == MUSIC_COMPOSITION_LABEL
	}
}
impl PartialEq<MusicCompositionIriOrLabel> for &str {
	fn eq(&self, other: &MusicCompositionIriOrLabel) -> bool {
		*self == MusicCompositionIri || *self == MUSIC_COMPOSITION_LABEL
	}
}
