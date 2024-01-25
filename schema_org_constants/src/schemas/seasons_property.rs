/// <https://schema.org/seasons>
#[deprecated = "This schema is superseded by <https://schema.org/season>."]
pub const SEASONS_PROPERTY_IRI_HTTP: &str = "http://schema.org/seasons";
/// <https://schema.org/seasons>
#[deprecated = "This schema is superseded by <https://schema.org/season>."]
pub const SEASONS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/seasons";
/// <https://schema.org/seasons>
#[deprecated = "This schema is superseded by <https://schema.org/season>."]
pub const SEASONS_PROPERTY_LABEL: &str = "seasons";
pub struct SeasonsPropertyIri;
impl PartialEq<&str> for SeasonsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEASONS_PROPERTY_IRI_HTTP || *other == SEASONS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SeasonsPropertyIri> for &str {
	fn eq(&self, other: &SeasonsPropertyIri) -> bool {
		*self == SEASONS_PROPERTY_IRI_HTTP || *self == SEASONS_PROPERTY_IRI_HTTPS
	}
}
pub struct SeasonsPropertyIriOrLabel;
impl PartialEq<&str> for SeasonsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeasonsPropertyIri || *other == SEASONS_PROPERTY_LABEL
	}
}
impl PartialEq<SeasonsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SeasonsPropertyIriOrLabel) -> bool {
		*self == SeasonsPropertyIri || *self == SEASONS_PROPERTY_LABEL
	}
}
