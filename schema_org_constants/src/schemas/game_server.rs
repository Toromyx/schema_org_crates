/// <https://schema.org/GameServer>
pub const GAME_SERVER_IRI_HTTP: &str = "http://schema.org/GameServer";
/// <https://schema.org/GameServer>
pub const GAME_SERVER_IRI_HTTPS: &str = "https://schema.org/GameServer";
/// <https://schema.org/GameServer>
pub const GAME_SERVER_LABEL: &str = "GameServer";
pub struct GameServerIri;
impl PartialEq<&str> for GameServerIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GAME_SERVER_IRI_HTTP || *other == GAME_SERVER_IRI_HTTPS
	}
}
impl PartialEq<GameServerIri> for &str {
	fn eq(&self, other: &GameServerIri) -> bool {
		*self == GAME_SERVER_IRI_HTTP || *self == GAME_SERVER_IRI_HTTPS
	}
}
pub struct GameServerIriOrLabel;
impl PartialEq<&str> for GameServerIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GameServerIri || *other == GAME_SERVER_LABEL
	}
}
impl PartialEq<GameServerIriOrLabel> for &str {
	fn eq(&self, other: &GameServerIriOrLabel) -> bool {
		*self == GameServerIri || *self == GAME_SERVER_LABEL
	}
}
