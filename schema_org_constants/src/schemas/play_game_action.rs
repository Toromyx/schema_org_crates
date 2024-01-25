/// <https://schema.org/PlayGameAction>
pub const PLAY_GAME_ACTION_IRI_HTTP: &str = "http://schema.org/PlayGameAction";
/// <https://schema.org/PlayGameAction>
pub const PLAY_GAME_ACTION_IRI_HTTPS: &str = "https://schema.org/PlayGameAction";
/// <https://schema.org/PlayGameAction>
pub const PLAY_GAME_ACTION_LABEL: &str = "PlayGameAction";
pub struct PlayGameActionIri;
impl PartialEq<&str> for PlayGameActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PLAY_GAME_ACTION_IRI_HTTP || *other == PLAY_GAME_ACTION_IRI_HTTPS
	}
}
impl PartialEq<PlayGameActionIri> for &str {
	fn eq(&self, other: &PlayGameActionIri) -> bool {
		*self == PLAY_GAME_ACTION_IRI_HTTP || *self == PLAY_GAME_ACTION_IRI_HTTPS
	}
}
pub struct PlayGameActionIriOrLabel;
impl PartialEq<&str> for PlayGameActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PlayGameActionIri || *other == PLAY_GAME_ACTION_LABEL
	}
}
impl PartialEq<PlayGameActionIriOrLabel> for &str {
	fn eq(&self, other: &PlayGameActionIriOrLabel) -> bool {
		*self == PlayGameActionIri || *self == PLAY_GAME_ACTION_LABEL
	}
}
