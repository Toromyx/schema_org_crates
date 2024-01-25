/// <https://schema.org/artMedium>
pub const ART_MEDIUM_PROPERTY_IRI_HTTP: &str = "http://schema.org/artMedium";
/// <https://schema.org/artMedium>
pub const ART_MEDIUM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/artMedium";
/// <https://schema.org/artMedium>
pub const ART_MEDIUM_PROPERTY_LABEL: &str = "artMedium";
pub struct ArtMediumPropertyIri;
impl PartialEq<&str> for ArtMediumPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ART_MEDIUM_PROPERTY_IRI_HTTP || *other == ART_MEDIUM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArtMediumPropertyIri> for &str {
	fn eq(&self, other: &ArtMediumPropertyIri) -> bool {
		*self == ART_MEDIUM_PROPERTY_IRI_HTTP || *self == ART_MEDIUM_PROPERTY_IRI_HTTPS
	}
}
pub struct ArtMediumPropertyIriOrLabel;
impl PartialEq<&str> for ArtMediumPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArtMediumPropertyIri || *other == ART_MEDIUM_PROPERTY_LABEL
	}
}
impl PartialEq<ArtMediumPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArtMediumPropertyIriOrLabel) -> bool {
		*self == ArtMediumPropertyIri || *self == ART_MEDIUM_PROPERTY_LABEL
	}
}
