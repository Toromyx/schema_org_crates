/// <https://schema.org/artform>
pub const ARTFORM_PROPERTY_IRI_HTTP: &str = "http://schema.org/artform";
/// <https://schema.org/artform>
pub const ARTFORM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/artform";
/// <https://schema.org/artform>
pub const ARTFORM_PROPERTY_LABEL: &str = "artform";
pub struct ArtformPropertyIri;
impl PartialEq<&str> for ArtformPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARTFORM_PROPERTY_IRI_HTTP || *other == ARTFORM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArtformPropertyIri> for &str {
	fn eq(&self, other: &ArtformPropertyIri) -> bool {
		*self == ARTFORM_PROPERTY_IRI_HTTP || *self == ARTFORM_PROPERTY_IRI_HTTPS
	}
}
pub struct ArtformPropertyIriOrLabel;
impl PartialEq<&str> for ArtformPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArtformPropertyIri || *other == ARTFORM_PROPERTY_LABEL
	}
}
impl PartialEq<ArtformPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArtformPropertyIriOrLabel) -> bool {
		*self == ArtformPropertyIri || *self == ARTFORM_PROPERTY_LABEL
	}
}
