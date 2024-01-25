/// <https://schema.org/City>
pub const CITY_IRI_HTTP: &str = "http://schema.org/City";
/// <https://schema.org/City>
pub const CITY_IRI_HTTPS: &str = "https://schema.org/City";
/// <https://schema.org/City>
pub const CITY_LABEL: &str = "City";
pub struct CityIri;
impl PartialEq<&str> for CityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CITY_IRI_HTTP || *other == CITY_IRI_HTTPS
	}
}
impl PartialEq<CityIri> for &str {
	fn eq(&self, other: &CityIri) -> bool {
		*self == CITY_IRI_HTTP || *self == CITY_IRI_HTTPS
	}
}
pub struct CityIriOrLabel;
impl PartialEq<&str> for CityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CityIri || *other == CITY_LABEL
	}
}
impl PartialEq<CityIriOrLabel> for &str {
	fn eq(&self, other: &CityIriOrLabel) -> bool {
		*self == CityIri || *self == CITY_LABEL
	}
}
