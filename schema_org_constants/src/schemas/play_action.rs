/// <https://schema.org/PlayAction>
pub const PLAY_ACTION_IRI_HTTP: &str = "http://schema.org/PlayAction";
/// <https://schema.org/PlayAction>
pub const PLAY_ACTION_IRI_HTTPS: &str = "https://schema.org/PlayAction";
/// <https://schema.org/PlayAction>
pub const PLAY_ACTION_LABEL: &str = "PlayAction";
pub struct PlayActionIri;
impl PartialEq<&str> for PlayActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PLAY_ACTION_IRI_HTTP || *other == PLAY_ACTION_IRI_HTTPS
	}
}
impl PartialEq<PlayActionIri> for &str {
	fn eq(&self, other: &PlayActionIri) -> bool {
		*self == PLAY_ACTION_IRI_HTTP || *self == PLAY_ACTION_IRI_HTTPS
	}
}
pub struct PlayActionIriOrLabel;
impl PartialEq<&str> for PlayActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PlayActionIri || *other == PLAY_ACTION_LABEL
	}
}
impl PartialEq<PlayActionIriOrLabel> for &str {
	fn eq(&self, other: &PlayActionIriOrLabel) -> bool {
		*self == PlayActionIri || *self == PLAY_ACTION_LABEL
	}
}
