/// <https://schema.org/ineligibleRegion>
pub const INELIGIBLE_REGION_PROPERTY_IRI_HTTP: &str = "http://schema.org/ineligibleRegion";
/// <https://schema.org/ineligibleRegion>
pub const INELIGIBLE_REGION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/ineligibleRegion";
/// <https://schema.org/ineligibleRegion>
pub const INELIGIBLE_REGION_PROPERTY_LABEL: &str = "ineligibleRegion";
pub struct IneligibleRegionPropertyIri;
impl PartialEq<&str> for IneligibleRegionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INELIGIBLE_REGION_PROPERTY_IRI_HTTP
			|| *other == INELIGIBLE_REGION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IneligibleRegionPropertyIri> for &str {
	fn eq(&self, other: &IneligibleRegionPropertyIri) -> bool {
		*self == INELIGIBLE_REGION_PROPERTY_IRI_HTTP
			|| *self == INELIGIBLE_REGION_PROPERTY_IRI_HTTPS
	}
}
pub struct IneligibleRegionPropertyIriOrLabel;
impl PartialEq<&str> for IneligibleRegionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IneligibleRegionPropertyIri || *other == INELIGIBLE_REGION_PROPERTY_LABEL
	}
}
impl PartialEq<IneligibleRegionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IneligibleRegionPropertyIriOrLabel) -> bool {
		*self == IneligibleRegionPropertyIri || *self == INELIGIBLE_REGION_PROPERTY_LABEL
	}
}
