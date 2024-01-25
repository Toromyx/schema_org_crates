/// <https://schema.org/elevation>
pub const ELEVATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/elevation";
/// <https://schema.org/elevation>
pub const ELEVATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/elevation";
/// <https://schema.org/elevation>
pub const ELEVATION_PROPERTY_LABEL: &str = "elevation";
pub struct ElevationPropertyIri;
impl PartialEq<&str> for ElevationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ELEVATION_PROPERTY_IRI_HTTP || *other == ELEVATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ElevationPropertyIri> for &str {
	fn eq(&self, other: &ElevationPropertyIri) -> bool {
		*self == ELEVATION_PROPERTY_IRI_HTTP || *self == ELEVATION_PROPERTY_IRI_HTTPS
	}
}
pub struct ElevationPropertyIriOrLabel;
impl PartialEq<&str> for ElevationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ElevationPropertyIri || *other == ELEVATION_PROPERTY_LABEL
	}
}
impl PartialEq<ElevationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ElevationPropertyIriOrLabel) -> bool {
		*self == ElevationPropertyIri || *self == ELEVATION_PROPERTY_LABEL
	}
}
