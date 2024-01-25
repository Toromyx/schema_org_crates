/// <https://schema.org/partOfSeason>
pub const PART_OF_SEASON_PROPERTY_IRI_HTTP: &str = "http://schema.org/partOfSeason";
/// <https://schema.org/partOfSeason>
pub const PART_OF_SEASON_PROPERTY_IRI_HTTPS: &str = "https://schema.org/partOfSeason";
/// <https://schema.org/partOfSeason>
pub const PART_OF_SEASON_PROPERTY_LABEL: &str = "partOfSeason";
pub struct PartOfSeasonPropertyIri;
impl PartialEq<&str> for PartOfSeasonPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PART_OF_SEASON_PROPERTY_IRI_HTTP || *other == PART_OF_SEASON_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PartOfSeasonPropertyIri> for &str {
	fn eq(&self, other: &PartOfSeasonPropertyIri) -> bool {
		*self == PART_OF_SEASON_PROPERTY_IRI_HTTP || *self == PART_OF_SEASON_PROPERTY_IRI_HTTPS
	}
}
pub struct PartOfSeasonPropertyIriOrLabel;
impl PartialEq<&str> for PartOfSeasonPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PartOfSeasonPropertyIri || *other == PART_OF_SEASON_PROPERTY_LABEL
	}
}
impl PartialEq<PartOfSeasonPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PartOfSeasonPropertyIriOrLabel) -> bool {
		*self == PartOfSeasonPropertyIri || *self == PART_OF_SEASON_PROPERTY_LABEL
	}
}
