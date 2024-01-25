/// <https://schema.org/playMode>
pub const PLAY_MODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/playMode";
/// <https://schema.org/playMode>
pub const PLAY_MODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/playMode";
/// <https://schema.org/playMode>
pub const PLAY_MODE_PROPERTY_LABEL: &str = "playMode";
pub struct PlayModePropertyIri;
impl PartialEq<&str> for PlayModePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PLAY_MODE_PROPERTY_IRI_HTTP || *other == PLAY_MODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PlayModePropertyIri> for &str {
	fn eq(&self, other: &PlayModePropertyIri) -> bool {
		*self == PLAY_MODE_PROPERTY_IRI_HTTP || *self == PLAY_MODE_PROPERTY_IRI_HTTPS
	}
}
pub struct PlayModePropertyIriOrLabel;
impl PartialEq<&str> for PlayModePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PlayModePropertyIri || *other == PLAY_MODE_PROPERTY_LABEL
	}
}
impl PartialEq<PlayModePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PlayModePropertyIriOrLabel) -> bool {
		*self == PlayModePropertyIri || *self == PLAY_MODE_PROPERTY_LABEL
	}
}
