/// <https://schema.org/Play>
pub const PLAY_IRI_HTTP: &str = "http://schema.org/Play";
/// <https://schema.org/Play>
pub const PLAY_IRI_HTTPS: &str = "https://schema.org/Play";
/// <https://schema.org/Play>
pub const PLAY_LABEL: &str = "Play";
pub struct PlayIri;
impl PartialEq<&str> for PlayIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PLAY_IRI_HTTP || *other == PLAY_IRI_HTTPS
	}
}
impl PartialEq<PlayIri> for &str {
	fn eq(&self, other: &PlayIri) -> bool {
		*self == PLAY_IRI_HTTP || *self == PLAY_IRI_HTTPS
	}
}
pub struct PlayIriOrLabel;
impl PartialEq<&str> for PlayIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PlayIri || *other == PLAY_LABEL
	}
}
impl PartialEq<PlayIriOrLabel> for &str {
	fn eq(&self, other: &PlayIriOrLabel) -> bool {
		*self == PlayIri || *self == PLAY_LABEL
	}
}
