/// <https://schema.org/GamePlayMode>
pub const GAME_PLAY_MODE_IRI_HTTP: &str = "http://schema.org/GamePlayMode";
/// <https://schema.org/GamePlayMode>
pub const GAME_PLAY_MODE_IRI_HTTPS: &str = "https://schema.org/GamePlayMode";
/// <https://schema.org/GamePlayMode>
pub const GAME_PLAY_MODE_LABEL: &str = "GamePlayMode";
pub struct GamePlayModeIri;
impl PartialEq<&str> for GamePlayModeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GAME_PLAY_MODE_IRI_HTTP || *other == GAME_PLAY_MODE_IRI_HTTPS
	}
}
impl PartialEq<GamePlayModeIri> for &str {
	fn eq(&self, other: &GamePlayModeIri) -> bool {
		*self == GAME_PLAY_MODE_IRI_HTTP || *self == GAME_PLAY_MODE_IRI_HTTPS
	}
}
pub struct GamePlayModeIriOrLabel;
impl PartialEq<&str> for GamePlayModeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GamePlayModeIri || *other == GAME_PLAY_MODE_LABEL
	}
}
impl PartialEq<GamePlayModeIriOrLabel> for &str {
	fn eq(&self, other: &GamePlayModeIriOrLabel) -> bool {
		*self == GamePlayModeIri || *self == GAME_PLAY_MODE_LABEL
	}
}
