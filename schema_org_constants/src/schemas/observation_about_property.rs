/// <https://schema.org/observationAbout>
pub const OBSERVATION_ABOUT_PROPERTY_IRI_HTTP: &str = "http://schema.org/observationAbout";
/// <https://schema.org/observationAbout>
pub const OBSERVATION_ABOUT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/observationAbout";
/// <https://schema.org/observationAbout>
pub const OBSERVATION_ABOUT_PROPERTY_LABEL: &str = "observationAbout";
pub struct ObservationAboutPropertyIri;
impl PartialEq<&str> for ObservationAboutPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OBSERVATION_ABOUT_PROPERTY_IRI_HTTP
			|| *other == OBSERVATION_ABOUT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ObservationAboutPropertyIri> for &str {
	fn eq(&self, other: &ObservationAboutPropertyIri) -> bool {
		*self == OBSERVATION_ABOUT_PROPERTY_IRI_HTTP
			|| *self == OBSERVATION_ABOUT_PROPERTY_IRI_HTTPS
	}
}
pub struct ObservationAboutPropertyIriOrLabel;
impl PartialEq<&str> for ObservationAboutPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ObservationAboutPropertyIri || *other == OBSERVATION_ABOUT_PROPERTY_LABEL
	}
}
impl PartialEq<ObservationAboutPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ObservationAboutPropertyIriOrLabel) -> bool {
		*self == ObservationAboutPropertyIri || *self == OBSERVATION_ABOUT_PROPERTY_LABEL
	}
}
