/// <https://schema.org/deathPlace>
pub const DEATH_PLACE_PROPERTY_IRI_HTTP: &str = "http://schema.org/deathPlace";
/// <https://schema.org/deathPlace>
pub const DEATH_PLACE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/deathPlace";
/// <https://schema.org/deathPlace>
pub const DEATH_PLACE_PROPERTY_LABEL: &str = "deathPlace";
pub struct DeathPlacePropertyIri;
impl PartialEq<&str> for DeathPlacePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEATH_PLACE_PROPERTY_IRI_HTTP || *other == DEATH_PLACE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DeathPlacePropertyIri> for &str {
	fn eq(&self, other: &DeathPlacePropertyIri) -> bool {
		*self == DEATH_PLACE_PROPERTY_IRI_HTTP || *self == DEATH_PLACE_PROPERTY_IRI_HTTPS
	}
}
pub struct DeathPlacePropertyIriOrLabel;
impl PartialEq<&str> for DeathPlacePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DeathPlacePropertyIri || *other == DEATH_PLACE_PROPERTY_LABEL
	}
}
impl PartialEq<DeathPlacePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DeathPlacePropertyIriOrLabel) -> bool {
		*self == DeathPlacePropertyIri || *self == DEATH_PLACE_PROPERTY_LABEL
	}
}
