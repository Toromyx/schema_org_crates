/// <https://schema.org/gameEdition>
pub const GAME_EDITION_PROPERTY_IRI_HTTP: &str = "http://schema.org/gameEdition";
/// <https://schema.org/gameEdition>
pub const GAME_EDITION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/gameEdition";
/// <https://schema.org/gameEdition>
pub const GAME_EDITION_PROPERTY_LABEL: &str = "gameEdition";
pub struct GameEditionPropertyIri;
impl PartialEq<&str> for GameEditionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GAME_EDITION_PROPERTY_IRI_HTTP || *other == GAME_EDITION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GameEditionPropertyIri> for &str {
	fn eq(&self, other: &GameEditionPropertyIri) -> bool {
		*self == GAME_EDITION_PROPERTY_IRI_HTTP || *self == GAME_EDITION_PROPERTY_IRI_HTTPS
	}
}
pub struct GameEditionPropertyIriOrLabel;
impl PartialEq<&str> for GameEditionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GameEditionPropertyIri || *other == GAME_EDITION_PROPERTY_LABEL
	}
}
impl PartialEq<GameEditionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GameEditionPropertyIriOrLabel) -> bool {
		*self == GameEditionPropertyIri || *self == GAME_EDITION_PROPERTY_LABEL
	}
}
