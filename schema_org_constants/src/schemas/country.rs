/// <https://schema.org/Country>
pub const COUNTRY_IRI_HTTP: &str = "http://schema.org/Country";
/// <https://schema.org/Country>
pub const COUNTRY_IRI_HTTPS: &str = "https://schema.org/Country";
/// <https://schema.org/Country>
pub const COUNTRY_LABEL: &str = "Country";
pub struct CountryIri;
impl PartialEq<&str> for CountryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COUNTRY_IRI_HTTP || *other == COUNTRY_IRI_HTTPS
	}
}
impl PartialEq<CountryIri> for &str {
	fn eq(&self, other: &CountryIri) -> bool {
		*self == COUNTRY_IRI_HTTP || *self == COUNTRY_IRI_HTTPS
	}
}
pub struct CountryIriOrLabel;
impl PartialEq<&str> for CountryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CountryIri || *other == COUNTRY_LABEL
	}
}
impl PartialEq<CountryIriOrLabel> for &str {
	fn eq(&self, other: &CountryIriOrLabel) -> bool {
		*self == CountryIri || *self == COUNTRY_LABEL
	}
}
