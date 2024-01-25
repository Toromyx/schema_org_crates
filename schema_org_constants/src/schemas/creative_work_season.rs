/// <https://schema.org/CreativeWorkSeason>
pub const CREATIVE_WORK_SEASON_IRI_HTTP: &str = "http://schema.org/CreativeWorkSeason";
/// <https://schema.org/CreativeWorkSeason>
pub const CREATIVE_WORK_SEASON_IRI_HTTPS: &str = "https://schema.org/CreativeWorkSeason";
/// <https://schema.org/CreativeWorkSeason>
pub const CREATIVE_WORK_SEASON_LABEL: &str = "CreativeWorkSeason";
pub struct CreativeWorkSeasonIri;
impl PartialEq<&str> for CreativeWorkSeasonIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CREATIVE_WORK_SEASON_IRI_HTTP || *other == CREATIVE_WORK_SEASON_IRI_HTTPS
	}
}
impl PartialEq<CreativeWorkSeasonIri> for &str {
	fn eq(&self, other: &CreativeWorkSeasonIri) -> bool {
		*self == CREATIVE_WORK_SEASON_IRI_HTTP || *self == CREATIVE_WORK_SEASON_IRI_HTTPS
	}
}
pub struct CreativeWorkSeasonIriOrLabel;
impl PartialEq<&str> for CreativeWorkSeasonIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CreativeWorkSeasonIri || *other == CREATIVE_WORK_SEASON_LABEL
	}
}
impl PartialEq<CreativeWorkSeasonIriOrLabel> for &str {
	fn eq(&self, other: &CreativeWorkSeasonIriOrLabel) -> bool {
		*self == CreativeWorkSeasonIri || *self == CREATIVE_WORK_SEASON_LABEL
	}
}
