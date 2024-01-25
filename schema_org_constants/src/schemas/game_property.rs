/// <https://schema.org/game>
pub const GAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/game";
/// <https://schema.org/game>
pub const GAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/game";
/// <https://schema.org/game>
pub const GAME_PROPERTY_LABEL: &str = "game";
pub struct GamePropertyIri;
impl PartialEq<&str> for GamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GAME_PROPERTY_IRI_HTTP || *other == GAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GamePropertyIri> for &str {
	fn eq(&self, other: &GamePropertyIri) -> bool {
		*self == GAME_PROPERTY_IRI_HTTP || *self == GAME_PROPERTY_IRI_HTTPS
	}
}
pub struct GamePropertyIriOrLabel;
impl PartialEq<&str> for GamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GamePropertyIri || *other == GAME_PROPERTY_LABEL
	}
}
impl PartialEq<GamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &GamePropertyIriOrLabel) -> bool {
		*self == GamePropertyIri || *self == GAME_PROPERTY_LABEL
	}
}
