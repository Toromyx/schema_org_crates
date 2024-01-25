/// <https://schema.org/season>
#[deprecated = "This schema is superseded by <https://schema.org/containsSeason>."]
pub const SEASON_PROPERTY_IRI_HTTP: &str = "http://schema.org/season";
/// <https://schema.org/season>
#[deprecated = "This schema is superseded by <https://schema.org/containsSeason>."]
pub const SEASON_PROPERTY_IRI_HTTPS: &str = "https://schema.org/season";
/// <https://schema.org/season>
#[deprecated = "This schema is superseded by <https://schema.org/containsSeason>."]
pub const SEASON_PROPERTY_LABEL: &str = "season";
pub struct SeasonPropertyIri;
impl PartialEq<&str> for SeasonPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEASON_PROPERTY_IRI_HTTP || *other == SEASON_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SeasonPropertyIri> for &str {
	fn eq(&self, other: &SeasonPropertyIri) -> bool {
		*self == SEASON_PROPERTY_IRI_HTTP || *self == SEASON_PROPERTY_IRI_HTTPS
	}
}
pub struct SeasonPropertyIriOrLabel;
impl PartialEq<&str> for SeasonPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeasonPropertyIri || *other == SEASON_PROPERTY_LABEL
	}
}
impl PartialEq<SeasonPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SeasonPropertyIriOrLabel) -> bool {
		*self == SeasonPropertyIri || *self == SEASON_PROPERTY_LABEL
	}
}
