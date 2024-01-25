/// <https://schema.org/playerType>
pub const PLAYER_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/playerType";
/// <https://schema.org/playerType>
pub const PLAYER_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/playerType";
/// <https://schema.org/playerType>
pub const PLAYER_TYPE_PROPERTY_LABEL: &str = "playerType";
pub struct PlayerTypePropertyIri;
impl PartialEq<&str> for PlayerTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PLAYER_TYPE_PROPERTY_IRI_HTTP || *other == PLAYER_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PlayerTypePropertyIri> for &str {
	fn eq(&self, other: &PlayerTypePropertyIri) -> bool {
		*self == PLAYER_TYPE_PROPERTY_IRI_HTTP || *self == PLAYER_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct PlayerTypePropertyIriOrLabel;
impl PartialEq<&str> for PlayerTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PlayerTypePropertyIri || *other == PLAYER_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<PlayerTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PlayerTypePropertyIriOrLabel) -> bool {
		*self == PlayerTypePropertyIri || *self == PLAYER_TYPE_PROPERTY_LABEL
	}
}
