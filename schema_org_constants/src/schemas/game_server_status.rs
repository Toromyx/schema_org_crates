/// <https://schema.org/GameServerStatus>
pub const GAME_SERVER_STATUS_IRI_HTTP: &str = "http://schema.org/GameServerStatus";
/// <https://schema.org/GameServerStatus>
pub const GAME_SERVER_STATUS_IRI_HTTPS: &str = "https://schema.org/GameServerStatus";
/// <https://schema.org/GameServerStatus>
pub const GAME_SERVER_STATUS_LABEL: &str = "GameServerStatus";
pub struct GameServerStatusIri;
impl PartialEq<&str> for GameServerStatusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GAME_SERVER_STATUS_IRI_HTTP || *other == GAME_SERVER_STATUS_IRI_HTTPS
	}
}
impl PartialEq<GameServerStatusIri> for &str {
	fn eq(&self, other: &GameServerStatusIri) -> bool {
		*self == GAME_SERVER_STATUS_IRI_HTTP || *self == GAME_SERVER_STATUS_IRI_HTTPS
	}
}
pub struct GameServerStatusIriOrLabel;
impl PartialEq<&str> for GameServerStatusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GameServerStatusIri || *other == GAME_SERVER_STATUS_LABEL
	}
}
impl PartialEq<GameServerStatusIriOrLabel> for &str {
	fn eq(&self, other: &GameServerStatusIriOrLabel) -> bool {
		*self == GameServerStatusIri || *self == GAME_SERVER_STATUS_LABEL
	}
}
