/// <https://schema.org/playersOnline>
pub const PLAYERS_ONLINE_PROPERTY_IRI_HTTP: &str = "http://schema.org/playersOnline";
/// <https://schema.org/playersOnline>
pub const PLAYERS_ONLINE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/playersOnline";
/// <https://schema.org/playersOnline>
pub const PLAYERS_ONLINE_PROPERTY_LABEL: &str = "playersOnline";
pub struct PlayersOnlinePropertyIri;
impl PartialEq<&str> for PlayersOnlinePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PLAYERS_ONLINE_PROPERTY_IRI_HTTP || *other == PLAYERS_ONLINE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PlayersOnlinePropertyIri> for &str {
	fn eq(&self, other: &PlayersOnlinePropertyIri) -> bool {
		*self == PLAYERS_ONLINE_PROPERTY_IRI_HTTP || *self == PLAYERS_ONLINE_PROPERTY_IRI_HTTPS
	}
}
pub struct PlayersOnlinePropertyIriOrLabel;
impl PartialEq<&str> for PlayersOnlinePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PlayersOnlinePropertyIri || *other == PLAYERS_ONLINE_PROPERTY_LABEL
	}
}
impl PartialEq<PlayersOnlinePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PlayersOnlinePropertyIriOrLabel) -> bool {
		*self == PlayersOnlinePropertyIri || *self == PLAYERS_ONLINE_PROPERTY_LABEL
	}
}
