/// <https://schema.org/DefinedRegion>
pub const DEFINED_REGION_IRI_HTTP: &str = "http://schema.org/DefinedRegion";
/// <https://schema.org/DefinedRegion>
pub const DEFINED_REGION_IRI_HTTPS: &str = "https://schema.org/DefinedRegion";
/// <https://schema.org/DefinedRegion>
pub const DEFINED_REGION_LABEL: &str = "DefinedRegion";
pub struct DefinedRegionIri;
impl PartialEq<&str> for DefinedRegionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEFINED_REGION_IRI_HTTP || *other == DEFINED_REGION_IRI_HTTPS
	}
}
impl PartialEq<DefinedRegionIri> for &str {
	fn eq(&self, other: &DefinedRegionIri) -> bool {
		*self == DEFINED_REGION_IRI_HTTP || *self == DEFINED_REGION_IRI_HTTPS
	}
}
pub struct DefinedRegionIriOrLabel;
impl PartialEq<&str> for DefinedRegionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DefinedRegionIri || *other == DEFINED_REGION_LABEL
	}
}
impl PartialEq<DefinedRegionIriOrLabel> for &str {
	fn eq(&self, other: &DefinedRegionIriOrLabel) -> bool {
		*self == DefinedRegionIri || *self == DEFINED_REGION_LABEL
	}
}
