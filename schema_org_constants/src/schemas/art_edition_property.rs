/// <https://schema.org/artEdition>
pub const ART_EDITION_PROPERTY_IRI_HTTP: &str = "http://schema.org/artEdition";
/// <https://schema.org/artEdition>
pub const ART_EDITION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/artEdition";
/// <https://schema.org/artEdition>
pub const ART_EDITION_PROPERTY_LABEL: &str = "artEdition";
pub struct ArtEditionPropertyIri;
impl PartialEq<&str> for ArtEditionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ART_EDITION_PROPERTY_IRI_HTTP || *other == ART_EDITION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArtEditionPropertyIri> for &str {
	fn eq(&self, other: &ArtEditionPropertyIri) -> bool {
		*self == ART_EDITION_PROPERTY_IRI_HTTP || *self == ART_EDITION_PROPERTY_IRI_HTTPS
	}
}
pub struct ArtEditionPropertyIriOrLabel;
impl PartialEq<&str> for ArtEditionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArtEditionPropertyIri || *other == ART_EDITION_PROPERTY_LABEL
	}
}
impl PartialEq<ArtEditionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArtEditionPropertyIriOrLabel) -> bool {
		*self == ArtEditionPropertyIri || *self == ART_EDITION_PROPERTY_LABEL
	}
}
