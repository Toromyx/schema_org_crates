/// <https://schema.org/spatialCoverage>
pub const SPATIAL_COVERAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/spatialCoverage";
/// <https://schema.org/spatialCoverage>
pub const SPATIAL_COVERAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/spatialCoverage";
/// <https://schema.org/spatialCoverage>
pub const SPATIAL_COVERAGE_PROPERTY_LABEL: &str = "spatialCoverage";
pub struct SpatialCoveragePropertyIri;
impl PartialEq<&str> for SpatialCoveragePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPATIAL_COVERAGE_PROPERTY_IRI_HTTP
			|| *other == SPATIAL_COVERAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SpatialCoveragePropertyIri> for &str {
	fn eq(&self, other: &SpatialCoveragePropertyIri) -> bool {
		*self == SPATIAL_COVERAGE_PROPERTY_IRI_HTTP || *self == SPATIAL_COVERAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct SpatialCoveragePropertyIriOrLabel;
impl PartialEq<&str> for SpatialCoveragePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpatialCoveragePropertyIri || *other == SPATIAL_COVERAGE_PROPERTY_LABEL
	}
}
impl PartialEq<SpatialCoveragePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SpatialCoveragePropertyIriOrLabel) -> bool {
		*self == SpatialCoveragePropertyIri || *self == SPATIAL_COVERAGE_PROPERTY_LABEL
	}
}
