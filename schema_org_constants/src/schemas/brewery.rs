/// <https://schema.org/Brewery>
pub const BREWERY_IRI_HTTP: &str = "http://schema.org/Brewery";
/// <https://schema.org/Brewery>
pub const BREWERY_IRI_HTTPS: &str = "https://schema.org/Brewery";
/// <https://schema.org/Brewery>
pub const BREWERY_LABEL: &str = "Brewery";
pub struct BreweryIri;
impl PartialEq<&str> for BreweryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BREWERY_IRI_HTTP || *other == BREWERY_IRI_HTTPS
	}
}
impl PartialEq<BreweryIri> for &str {
	fn eq(&self, other: &BreweryIri) -> bool {
		*self == BREWERY_IRI_HTTP || *self == BREWERY_IRI_HTTPS
	}
}
pub struct BreweryIriOrLabel;
impl PartialEq<&str> for BreweryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BreweryIri || *other == BREWERY_LABEL
	}
}
impl PartialEq<BreweryIriOrLabel> for &str {
	fn eq(&self, other: &BreweryIriOrLabel) -> bool {
		*self == BreweryIri || *self == BREWERY_LABEL
	}
}
