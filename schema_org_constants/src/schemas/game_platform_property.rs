/// <https://schema.org/gamePlatform>
pub const GAME_PLATFORM_PROPERTY_IRI_HTTP: &str = "http://schema.org/gamePlatform";
/// <https://schema.org/gamePlatform>
pub const GAME_PLATFORM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/gamePlatform";
/// <https://schema.org/gamePlatform>
pub const GAME_PLATFORM_PROPERTY_LABEL: &str = "gamePlatform";
pub struct GamePlatformPropertyIri;
impl PartialEq<&str> for GamePlatformPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GAME_PLATFORM_PROPERTY_IRI_HTTP || *other == GAME_PLATFORM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GamePlatformPropertyIri> for &str {
	fn eq(&self, other: &GamePlatformPropertyIri) -> bool {
		*self == GAME_PLATFORM_PROPERTY_IRI_HTTP || *self == GAME_PLATFORM_PROPERTY_IRI_HTTPS
	}
}
pub struct GamePlatformPropertyIriOrLabel;
impl PartialEq<&str> for GamePlatformPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GamePlatformPropertyIri || *other == GAME_PLATFORM_PROPERTY_LABEL
	}
}
impl PartialEq<GamePlatformPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GamePlatformPropertyIriOrLabel) -> bool {
		*self == GamePlatformPropertyIri || *self == GAME_PLATFORM_PROPERTY_LABEL
	}
}
