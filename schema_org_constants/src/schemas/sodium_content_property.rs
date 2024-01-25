/// <https://schema.org/sodiumContent>
pub const SODIUM_CONTENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/sodiumContent";
/// <https://schema.org/sodiumContent>
pub const SODIUM_CONTENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sodiumContent";
/// <https://schema.org/sodiumContent>
pub const SODIUM_CONTENT_PROPERTY_LABEL: &str = "sodiumContent";
pub struct SodiumContentPropertyIri;
impl PartialEq<&str> for SodiumContentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SODIUM_CONTENT_PROPERTY_IRI_HTTP || *other == SODIUM_CONTENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SodiumContentPropertyIri> for &str {
	fn eq(&self, other: &SodiumContentPropertyIri) -> bool {
		*self == SODIUM_CONTENT_PROPERTY_IRI_HTTP || *self == SODIUM_CONTENT_PROPERTY_IRI_HTTPS
	}
}
pub struct SodiumContentPropertyIriOrLabel;
impl PartialEq<&str> for SodiumContentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SodiumContentPropertyIri || *other == SODIUM_CONTENT_PROPERTY_LABEL
	}
}
impl PartialEq<SodiumContentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SodiumContentPropertyIriOrLabel) -> bool {
		*self == SodiumContentPropertyIri || *self == SODIUM_CONTENT_PROPERTY_LABEL
	}
}
