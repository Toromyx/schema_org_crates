/// <https://schema.org/containsSeason>
pub const CONTAINS_SEASON_PROPERTY_IRI_HTTP: &str = "http://schema.org/containsSeason";
/// <https://schema.org/containsSeason>
pub const CONTAINS_SEASON_PROPERTY_IRI_HTTPS: &str = "https://schema.org/containsSeason";
/// <https://schema.org/containsSeason>
pub const CONTAINS_SEASON_PROPERTY_LABEL: &str = "containsSeason";
pub struct ContainsSeasonPropertyIri;
impl PartialEq<&str> for ContainsSeasonPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTAINS_SEASON_PROPERTY_IRI_HTTP || *other == CONTAINS_SEASON_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContainsSeasonPropertyIri> for &str {
	fn eq(&self, other: &ContainsSeasonPropertyIri) -> bool {
		*self == CONTAINS_SEASON_PROPERTY_IRI_HTTP || *self == CONTAINS_SEASON_PROPERTY_IRI_HTTPS
	}
}
pub struct ContainsSeasonPropertyIriOrLabel;
impl PartialEq<&str> for ContainsSeasonPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContainsSeasonPropertyIri || *other == CONTAINS_SEASON_PROPERTY_LABEL
	}
}
impl PartialEq<ContainsSeasonPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContainsSeasonPropertyIriOrLabel) -> bool {
		*self == ContainsSeasonPropertyIri || *self == CONTAINS_SEASON_PROPERTY_LABEL
	}
}
