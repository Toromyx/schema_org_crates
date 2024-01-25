/// <https://schema.org/Continent>
pub const CONTINENT_IRI_HTTP: &str = "http://schema.org/Continent";
/// <https://schema.org/Continent>
pub const CONTINENT_IRI_HTTPS: &str = "https://schema.org/Continent";
/// <https://schema.org/Continent>
pub const CONTINENT_LABEL: &str = "Continent";
pub struct ContinentIri;
impl PartialEq<&str> for ContinentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTINENT_IRI_HTTP || *other == CONTINENT_IRI_HTTPS
	}
}
impl PartialEq<ContinentIri> for &str {
	fn eq(&self, other: &ContinentIri) -> bool {
		*self == CONTINENT_IRI_HTTP || *self == CONTINENT_IRI_HTTPS
	}
}
pub struct ContinentIriOrLabel;
impl PartialEq<&str> for ContinentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContinentIri || *other == CONTINENT_LABEL
	}
}
impl PartialEq<ContinentIriOrLabel> for &str {
	fn eq(&self, other: &ContinentIriOrLabel) -> bool {
		*self == ContinentIri || *self == CONTINENT_LABEL
	}
}
