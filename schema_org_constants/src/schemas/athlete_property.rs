/// <https://schema.org/athlete>
pub const ATHLETE_PROPERTY_IRI_HTTP: &str = "http://schema.org/athlete";
/// <https://schema.org/athlete>
pub const ATHLETE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/athlete";
/// <https://schema.org/athlete>
pub const ATHLETE_PROPERTY_LABEL: &str = "athlete";
pub struct AthletePropertyIri;
impl PartialEq<&str> for AthletePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ATHLETE_PROPERTY_IRI_HTTP || *other == ATHLETE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AthletePropertyIri> for &str {
	fn eq(&self, other: &AthletePropertyIri) -> bool {
		*self == ATHLETE_PROPERTY_IRI_HTTP || *self == ATHLETE_PROPERTY_IRI_HTTPS
	}
}
pub struct AthletePropertyIriOrLabel;
impl PartialEq<&str> for AthletePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AthletePropertyIri || *other == ATHLETE_PROPERTY_LABEL
	}
}
impl PartialEq<AthletePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AthletePropertyIriOrLabel) -> bool {
		*self == AthletePropertyIri || *self == ATHLETE_PROPERTY_LABEL
	}
}
