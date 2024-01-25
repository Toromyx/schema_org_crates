/// <https://schema.org/GeoCircle>
pub const GEO_CIRCLE_IRI_HTTP: &str = "http://schema.org/GeoCircle";
/// <https://schema.org/GeoCircle>
pub const GEO_CIRCLE_IRI_HTTPS: &str = "https://schema.org/GeoCircle";
/// <https://schema.org/GeoCircle>
pub const GEO_CIRCLE_LABEL: &str = "GeoCircle";
pub struct GeoCircleIri;
impl PartialEq<&str> for GeoCircleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_CIRCLE_IRI_HTTP || *other == GEO_CIRCLE_IRI_HTTPS
	}
}
impl PartialEq<GeoCircleIri> for &str {
	fn eq(&self, other: &GeoCircleIri) -> bool {
		*self == GEO_CIRCLE_IRI_HTTP || *self == GEO_CIRCLE_IRI_HTTPS
	}
}
pub struct GeoCircleIriOrLabel;
impl PartialEq<&str> for GeoCircleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoCircleIri || *other == GEO_CIRCLE_LABEL
	}
}
impl PartialEq<GeoCircleIriOrLabel> for &str {
	fn eq(&self, other: &GeoCircleIriOrLabel) -> bool {
		*self == GeoCircleIri || *self == GEO_CIRCLE_LABEL
	}
}
