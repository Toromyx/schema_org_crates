/// <https://schema.org/Season>
#[deprecated = "This schema is superseded by <https://schema.org/CreativeWorkSeason>."]
pub const SEASON_IRI_HTTP: &str = "http://schema.org/Season";
/// <https://schema.org/Season>
#[deprecated = "This schema is superseded by <https://schema.org/CreativeWorkSeason>."]
pub const SEASON_IRI_HTTPS: &str = "https://schema.org/Season";
/// <https://schema.org/Season>
#[deprecated = "This schema is superseded by <https://schema.org/CreativeWorkSeason>."]
pub const SEASON_LABEL: &str = "Season";
pub struct SeasonIri;
impl PartialEq<&str> for SeasonIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEASON_IRI_HTTP || *other == SEASON_IRI_HTTPS
	}
}
impl PartialEq<SeasonIri> for &str {
	fn eq(&self, other: &SeasonIri) -> bool {
		*self == SEASON_IRI_HTTP || *self == SEASON_IRI_HTTPS
	}
}
pub struct SeasonIriOrLabel;
impl PartialEq<&str> for SeasonIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeasonIri || *other == SEASON_LABEL
	}
}
impl PartialEq<SeasonIriOrLabel> for &str {
	fn eq(&self, other: &SeasonIriOrLabel) -> bool {
		*self == SeasonIri || *self == SEASON_LABEL
	}
}
