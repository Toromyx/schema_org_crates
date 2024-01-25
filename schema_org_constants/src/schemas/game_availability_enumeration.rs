/// <https://schema.org/GameAvailabilityEnumeration>
pub const GAME_AVAILABILITY_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/GameAvailabilityEnumeration";
/// <https://schema.org/GameAvailabilityEnumeration>
pub const GAME_AVAILABILITY_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/GameAvailabilityEnumeration";
/// <https://schema.org/GameAvailabilityEnumeration>
pub const GAME_AVAILABILITY_ENUMERATION_LABEL: &str = "GameAvailabilityEnumeration";
pub struct GameAvailabilityEnumerationIri;
impl PartialEq<&str> for GameAvailabilityEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GAME_AVAILABILITY_ENUMERATION_IRI_HTTP
			|| *other == GAME_AVAILABILITY_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<GameAvailabilityEnumerationIri> for &str {
	fn eq(&self, other: &GameAvailabilityEnumerationIri) -> bool {
		*self == GAME_AVAILABILITY_ENUMERATION_IRI_HTTP
			|| *self == GAME_AVAILABILITY_ENUMERATION_IRI_HTTPS
	}
}
pub struct GameAvailabilityEnumerationIriOrLabel;
impl PartialEq<&str> for GameAvailabilityEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GameAvailabilityEnumerationIri || *other == GAME_AVAILABILITY_ENUMERATION_LABEL
	}
}
impl PartialEq<GameAvailabilityEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &GameAvailabilityEnumerationIriOrLabel) -> bool {
		*self == GameAvailabilityEnumerationIri || *self == GAME_AVAILABILITY_ENUMERATION_LABEL
	}
}
