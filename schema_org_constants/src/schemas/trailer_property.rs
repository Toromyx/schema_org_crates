/// <https://schema.org/trailer>
pub const TRAILER_PROPERTY_IRI_HTTP: &str = "http://schema.org/trailer";
/// <https://schema.org/trailer>
pub const TRAILER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/trailer";
/// <https://schema.org/trailer>
pub const TRAILER_PROPERTY_LABEL: &str = "trailer";
pub struct TrailerPropertyIri;
impl PartialEq<&str> for TrailerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRAILER_PROPERTY_IRI_HTTP || *other == TRAILER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TrailerPropertyIri> for &str {
	fn eq(&self, other: &TrailerPropertyIri) -> bool {
		*self == TRAILER_PROPERTY_IRI_HTTP || *self == TRAILER_PROPERTY_IRI_HTTPS
	}
}
pub struct TrailerPropertyIriOrLabel;
impl PartialEq<&str> for TrailerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrailerPropertyIri || *other == TRAILER_PROPERTY_LABEL
	}
}
impl PartialEq<TrailerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TrailerPropertyIriOrLabel) -> bool {
		*self == TrailerPropertyIri || *self == TRAILER_PROPERTY_LABEL
	}
}
