/// <https://schema.org/numberOfPlayers>
pub const NUMBER_OF_PLAYERS_PROPERTY_IRI_HTTP: &str = "http://schema.org/numberOfPlayers";
/// <https://schema.org/numberOfPlayers>
pub const NUMBER_OF_PLAYERS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numberOfPlayers";
/// <https://schema.org/numberOfPlayers>
pub const NUMBER_OF_PLAYERS_PROPERTY_LABEL: &str = "numberOfPlayers";
pub struct NumberOfPlayersPropertyIri;
impl PartialEq<&str> for NumberOfPlayersPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_PLAYERS_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_PLAYERS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfPlayersPropertyIri> for &str {
	fn eq(&self, other: &NumberOfPlayersPropertyIri) -> bool {
		*self == NUMBER_OF_PLAYERS_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_PLAYERS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfPlayersPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfPlayersPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfPlayersPropertyIri || *other == NUMBER_OF_PLAYERS_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfPlayersPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfPlayersPropertyIriOrLabel) -> bool {
		*self == NumberOfPlayersPropertyIri || *self == NUMBER_OF_PLAYERS_PROPERTY_LABEL
	}
}
