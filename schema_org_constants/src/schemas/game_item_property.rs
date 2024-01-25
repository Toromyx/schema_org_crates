/// <https://schema.org/gameItem>
pub const GAME_ITEM_PROPERTY_IRI_HTTP: &str = "http://schema.org/gameItem";
/// <https://schema.org/gameItem>
pub const GAME_ITEM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/gameItem";
/// <https://schema.org/gameItem>
pub const GAME_ITEM_PROPERTY_LABEL: &str = "gameItem";
pub struct GameItemPropertyIri;
impl PartialEq<&str> for GameItemPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GAME_ITEM_PROPERTY_IRI_HTTP || *other == GAME_ITEM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GameItemPropertyIri> for &str {
	fn eq(&self, other: &GameItemPropertyIri) -> bool {
		*self == GAME_ITEM_PROPERTY_IRI_HTTP || *self == GAME_ITEM_PROPERTY_IRI_HTTPS
	}
}
pub struct GameItemPropertyIriOrLabel;
impl PartialEq<&str> for GameItemPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GameItemPropertyIri || *other == GAME_ITEM_PROPERTY_LABEL
	}
}
impl PartialEq<GameItemPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GameItemPropertyIriOrLabel) -> bool {
		*self == GameItemPropertyIri || *self == GAME_ITEM_PROPERTY_LABEL
	}
}
