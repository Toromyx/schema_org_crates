/// <https://schema.org/sport>
pub const SPORT_PROPERTY_IRI_HTTP: &str = "http://schema.org/sport";
/// <https://schema.org/sport>
pub const SPORT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sport";
/// <https://schema.org/sport>
pub const SPORT_PROPERTY_LABEL: &str = "sport";
pub struct SportPropertyIri;
impl PartialEq<&str> for SportPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPORT_PROPERTY_IRI_HTTP || *other == SPORT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SportPropertyIri> for &str {
	fn eq(&self, other: &SportPropertyIri) -> bool {
		*self == SPORT_PROPERTY_IRI_HTTP || *self == SPORT_PROPERTY_IRI_HTTPS
	}
}
pub struct SportPropertyIriOrLabel;
impl PartialEq<&str> for SportPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SportPropertyIri || *other == SPORT_PROPERTY_LABEL
	}
}
impl PartialEq<SportPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SportPropertyIriOrLabel) -> bool {
		*self == SportPropertyIri || *self == SPORT_PROPERTY_LABEL
	}
}
