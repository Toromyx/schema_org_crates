/// <https://schema.org/gameTip>
pub const GAME_TIP_PROPERTY_IRI_HTTP: &str = "http://schema.org/gameTip";
/// <https://schema.org/gameTip>
pub const GAME_TIP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/gameTip";
/// <https://schema.org/gameTip>
pub const GAME_TIP_PROPERTY_LABEL: &str = "gameTip";
pub struct GameTipPropertyIri;
impl PartialEq<&str> for GameTipPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GAME_TIP_PROPERTY_IRI_HTTP || *other == GAME_TIP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GameTipPropertyIri> for &str {
	fn eq(&self, other: &GameTipPropertyIri) -> bool {
		*self == GAME_TIP_PROPERTY_IRI_HTTP || *self == GAME_TIP_PROPERTY_IRI_HTTPS
	}
}
pub struct GameTipPropertyIriOrLabel;
impl PartialEq<&str> for GameTipPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GameTipPropertyIri || *other == GAME_TIP_PROPERTY_LABEL
	}
}
impl PartialEq<GameTipPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GameTipPropertyIriOrLabel) -> bool {
		*self == GameTipPropertyIri || *self == GAME_TIP_PROPERTY_LABEL
	}
}
