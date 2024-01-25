/// <https://schema.org/deathDate>
pub const DEATH_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/deathDate";
/// <https://schema.org/deathDate>
pub const DEATH_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/deathDate";
/// <https://schema.org/deathDate>
pub const DEATH_DATE_PROPERTY_LABEL: &str = "deathDate";
pub struct DeathDatePropertyIri;
impl PartialEq<&str> for DeathDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEATH_DATE_PROPERTY_IRI_HTTP || *other == DEATH_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DeathDatePropertyIri> for &str {
	fn eq(&self, other: &DeathDatePropertyIri) -> bool {
		*self == DEATH_DATE_PROPERTY_IRI_HTTP || *self == DEATH_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct DeathDatePropertyIriOrLabel;
impl PartialEq<&str> for DeathDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DeathDatePropertyIri || *other == DEATH_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<DeathDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DeathDatePropertyIriOrLabel) -> bool {
		*self == DeathDatePropertyIri || *self == DEATH_DATE_PROPERTY_LABEL
	}
}
