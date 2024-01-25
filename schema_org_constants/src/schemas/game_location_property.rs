/// <https://schema.org/gameLocation>
pub const GAME_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/gameLocation";
/// <https://schema.org/gameLocation>
pub const GAME_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/gameLocation";
/// <https://schema.org/gameLocation>
pub const GAME_LOCATION_PROPERTY_LABEL: &str = "gameLocation";
pub struct GameLocationPropertyIri;
impl PartialEq<&str> for GameLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GAME_LOCATION_PROPERTY_IRI_HTTP || *other == GAME_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GameLocationPropertyIri> for &str {
	fn eq(&self, other: &GameLocationPropertyIri) -> bool {
		*self == GAME_LOCATION_PROPERTY_IRI_HTTP || *self == GAME_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct GameLocationPropertyIriOrLabel;
impl PartialEq<&str> for GameLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GameLocationPropertyIri || *other == GAME_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<GameLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GameLocationPropertyIriOrLabel) -> bool {
		*self == GameLocationPropertyIri || *self == GAME_LOCATION_PROPERTY_LABEL
	}
}
