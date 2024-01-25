/// <https://schema.org/Game>
pub const GAME_IRI_HTTP: &str = "http://schema.org/Game";
/// <https://schema.org/Game>
pub const GAME_IRI_HTTPS: &str = "https://schema.org/Game";
/// <https://schema.org/Game>
pub const GAME_LABEL: &str = "Game";
pub struct GameIri;
impl PartialEq<&str> for GameIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GAME_IRI_HTTP || *other == GAME_IRI_HTTPS
	}
}
impl PartialEq<GameIri> for &str {
	fn eq(&self, other: &GameIri) -> bool {
		*self == GAME_IRI_HTTP || *self == GAME_IRI_HTTPS
	}
}
pub struct GameIriOrLabel;
impl PartialEq<&str> for GameIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GameIri || *other == GAME_LABEL
	}
}
impl PartialEq<GameIriOrLabel> for &str {
	fn eq(&self, other: &GameIriOrLabel) -> bool {
		*self == GameIri || *self == GAME_LABEL
	}
}
