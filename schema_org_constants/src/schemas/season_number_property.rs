/// <https://schema.org/seasonNumber>
pub const SEASON_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/seasonNumber";
/// <https://schema.org/seasonNumber>
pub const SEASON_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/seasonNumber";
/// <https://schema.org/seasonNumber>
pub const SEASON_NUMBER_PROPERTY_LABEL: &str = "seasonNumber";
pub struct SeasonNumberPropertyIri;
impl PartialEq<&str> for SeasonNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEASON_NUMBER_PROPERTY_IRI_HTTP || *other == SEASON_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SeasonNumberPropertyIri> for &str {
	fn eq(&self, other: &SeasonNumberPropertyIri) -> bool {
		*self == SEASON_NUMBER_PROPERTY_IRI_HTTP || *self == SEASON_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct SeasonNumberPropertyIriOrLabel;
impl PartialEq<&str> for SeasonNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeasonNumberPropertyIri || *other == SEASON_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<SeasonNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SeasonNumberPropertyIriOrLabel) -> bool {
		*self == SeasonNumberPropertyIri || *self == SEASON_NUMBER_PROPERTY_LABEL
	}
}
