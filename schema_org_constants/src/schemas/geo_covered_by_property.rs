/// <https://schema.org/geoCoveredBy>
pub const GEO_COVERED_BY_PROPERTY_IRI_HTTP: &str = "http://schema.org/geoCoveredBy";
/// <https://schema.org/geoCoveredBy>
pub const GEO_COVERED_BY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/geoCoveredBy";
/// <https://schema.org/geoCoveredBy>
pub const GEO_COVERED_BY_PROPERTY_LABEL: &str = "geoCoveredBy";
pub struct GeoCoveredByPropertyIri;
impl PartialEq<&str> for GeoCoveredByPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_COVERED_BY_PROPERTY_IRI_HTTP || *other == GEO_COVERED_BY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GeoCoveredByPropertyIri> for &str {
	fn eq(&self, other: &GeoCoveredByPropertyIri) -> bool {
		*self == GEO_COVERED_BY_PROPERTY_IRI_HTTP || *self == GEO_COVERED_BY_PROPERTY_IRI_HTTPS
	}
}
pub struct GeoCoveredByPropertyIriOrLabel;
impl PartialEq<&str> for GeoCoveredByPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoCoveredByPropertyIri || *other == GEO_COVERED_BY_PROPERTY_LABEL
	}
}
impl PartialEq<GeoCoveredByPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GeoCoveredByPropertyIriOrLabel) -> bool {
		*self == GeoCoveredByPropertyIri || *self == GEO_COVERED_BY_PROPERTY_LABEL
	}
}
