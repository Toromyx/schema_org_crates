/// <https://schema.org/gameAvailabilityType>
pub const GAME_AVAILABILITY_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/gameAvailabilityType";
/// <https://schema.org/gameAvailabilityType>
pub const GAME_AVAILABILITY_TYPE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/gameAvailabilityType";
/// <https://schema.org/gameAvailabilityType>
pub const GAME_AVAILABILITY_TYPE_PROPERTY_LABEL: &str = "gameAvailabilityType";
pub struct GameAvailabilityTypePropertyIri;
impl PartialEq<&str> for GameAvailabilityTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GAME_AVAILABILITY_TYPE_PROPERTY_IRI_HTTP
			|| *other == GAME_AVAILABILITY_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GameAvailabilityTypePropertyIri> for &str {
	fn eq(&self, other: &GameAvailabilityTypePropertyIri) -> bool {
		*self == GAME_AVAILABILITY_TYPE_PROPERTY_IRI_HTTP
			|| *self == GAME_AVAILABILITY_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct GameAvailabilityTypePropertyIriOrLabel;
impl PartialEq<&str> for GameAvailabilityTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GameAvailabilityTypePropertyIri || *other == GAME_AVAILABILITY_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<GameAvailabilityTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &GameAvailabilityTypePropertyIriOrLabel) -> bool {
		*self == GameAvailabilityTypePropertyIri || *self == GAME_AVAILABILITY_TYPE_PROPERTY_LABEL
	}
}
