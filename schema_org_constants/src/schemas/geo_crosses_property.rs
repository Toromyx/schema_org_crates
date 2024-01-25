/// <https://schema.org/geoCrosses>
pub const GEO_CROSSES_PROPERTY_IRI_HTTP: &str = "http://schema.org/geoCrosses";
/// <https://schema.org/geoCrosses>
pub const GEO_CROSSES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/geoCrosses";
/// <https://schema.org/geoCrosses>
pub const GEO_CROSSES_PROPERTY_LABEL: &str = "geoCrosses";
pub struct GeoCrossesPropertyIri;
impl PartialEq<&str> for GeoCrossesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_CROSSES_PROPERTY_IRI_HTTP || *other == GEO_CROSSES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GeoCrossesPropertyIri> for &str {
	fn eq(&self, other: &GeoCrossesPropertyIri) -> bool {
		*self == GEO_CROSSES_PROPERTY_IRI_HTTP || *self == GEO_CROSSES_PROPERTY_IRI_HTTPS
	}
}
pub struct GeoCrossesPropertyIriOrLabel;
impl PartialEq<&str> for GeoCrossesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoCrossesPropertyIri || *other == GEO_CROSSES_PROPERTY_LABEL
	}
}
impl PartialEq<GeoCrossesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GeoCrossesPropertyIriOrLabel) -> bool {
		*self == GeoCrossesPropertyIri || *self == GEO_CROSSES_PROPERTY_LABEL
	}
}
