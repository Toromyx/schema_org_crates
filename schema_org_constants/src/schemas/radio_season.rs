/// <https://schema.org/RadioSeason>
pub const RADIO_SEASON_IRI_HTTP: &str = "http://schema.org/RadioSeason";
/// <https://schema.org/RadioSeason>
pub const RADIO_SEASON_IRI_HTTPS: &str = "https://schema.org/RadioSeason";
/// <https://schema.org/RadioSeason>
pub const RADIO_SEASON_LABEL: &str = "RadioSeason";
pub struct RadioSeasonIri;
impl PartialEq<&str> for RadioSeasonIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RADIO_SEASON_IRI_HTTP || *other == RADIO_SEASON_IRI_HTTPS
	}
}
impl PartialEq<RadioSeasonIri> for &str {
	fn eq(&self, other: &RadioSeasonIri) -> bool {
		*self == RADIO_SEASON_IRI_HTTP || *self == RADIO_SEASON_IRI_HTTPS
	}
}
pub struct RadioSeasonIriOrLabel;
impl PartialEq<&str> for RadioSeasonIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RadioSeasonIri || *other == RADIO_SEASON_LABEL
	}
}
impl PartialEq<RadioSeasonIriOrLabel> for &str {
	fn eq(&self, other: &RadioSeasonIriOrLabel) -> bool {
		*self == RadioSeasonIri || *self == RADIO_SEASON_LABEL
	}
}
