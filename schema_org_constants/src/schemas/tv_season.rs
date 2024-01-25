/// <https://schema.org/TVSeason>
pub const TV_SEASON_IRI_HTTP: &str = "http://schema.org/TVSeason";
/// <https://schema.org/TVSeason>
pub const TV_SEASON_IRI_HTTPS: &str = "https://schema.org/TVSeason";
/// <https://schema.org/TVSeason>
pub const TV_SEASON_LABEL: &str = "TVSeason";
pub struct TvSeasonIri;
impl PartialEq<&str> for TvSeasonIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TV_SEASON_IRI_HTTP || *other == TV_SEASON_IRI_HTTPS
	}
}
impl PartialEq<TvSeasonIri> for &str {
	fn eq(&self, other: &TvSeasonIri) -> bool {
		*self == TV_SEASON_IRI_HTTP || *self == TV_SEASON_IRI_HTTPS
	}
}
pub struct TvSeasonIriOrLabel;
impl PartialEq<&str> for TvSeasonIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TvSeasonIri || *other == TV_SEASON_LABEL
	}
}
impl PartialEq<TvSeasonIriOrLabel> for &str {
	fn eq(&self, other: &TvSeasonIriOrLabel) -> bool {
		*self == TvSeasonIri || *self == TV_SEASON_LABEL
	}
}
