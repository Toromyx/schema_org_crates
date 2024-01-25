/// <https://schema.org/slogan>
pub const SLOGAN_PROPERTY_IRI_HTTP: &str = "http://schema.org/slogan";
/// <https://schema.org/slogan>
pub const SLOGAN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/slogan";
/// <https://schema.org/slogan>
pub const SLOGAN_PROPERTY_LABEL: &str = "slogan";
pub struct SloganPropertyIri;
impl PartialEq<&str> for SloganPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SLOGAN_PROPERTY_IRI_HTTP || *other == SLOGAN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SloganPropertyIri> for &str {
	fn eq(&self, other: &SloganPropertyIri) -> bool {
		*self == SLOGAN_PROPERTY_IRI_HTTP || *self == SLOGAN_PROPERTY_IRI_HTTPS
	}
}
pub struct SloganPropertyIriOrLabel;
impl PartialEq<&str> for SloganPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SloganPropertyIri || *other == SLOGAN_PROPERTY_LABEL
	}
}
impl PartialEq<SloganPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SloganPropertyIriOrLabel) -> bool {
		*self == SloganPropertyIri || *self == SLOGAN_PROPERTY_LABEL
	}
}
