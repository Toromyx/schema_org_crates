/// <https://schema.org/gameServer>
pub const GAME_SERVER_PROPERTY_IRI_HTTP: &str = "http://schema.org/gameServer";
/// <https://schema.org/gameServer>
pub const GAME_SERVER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/gameServer";
/// <https://schema.org/gameServer>
pub const GAME_SERVER_PROPERTY_LABEL: &str = "gameServer";
pub struct GameServerPropertyIri;
impl PartialEq<&str> for GameServerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GAME_SERVER_PROPERTY_IRI_HTTP || *other == GAME_SERVER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GameServerPropertyIri> for &str {
	fn eq(&self, other: &GameServerPropertyIri) -> bool {
		*self == GAME_SERVER_PROPERTY_IRI_HTTP || *self == GAME_SERVER_PROPERTY_IRI_HTTPS
	}
}
pub struct GameServerPropertyIriOrLabel;
impl PartialEq<&str> for GameServerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GameServerPropertyIri || *other == GAME_SERVER_PROPERTY_LABEL
	}
}
impl PartialEq<GameServerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GameServerPropertyIriOrLabel) -> bool {
		*self == GameServerPropertyIri || *self == GAME_SERVER_PROPERTY_LABEL
	}
}
