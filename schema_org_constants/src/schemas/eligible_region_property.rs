/// <https://schema.org/eligibleRegion>
pub const ELIGIBLE_REGION_PROPERTY_IRI_HTTP: &str = "http://schema.org/eligibleRegion";
/// <https://schema.org/eligibleRegion>
pub const ELIGIBLE_REGION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/eligibleRegion";
/// <https://schema.org/eligibleRegion>
pub const ELIGIBLE_REGION_PROPERTY_LABEL: &str = "eligibleRegion";
pub struct EligibleRegionPropertyIri;
impl PartialEq<&str> for EligibleRegionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ELIGIBLE_REGION_PROPERTY_IRI_HTTP || *other == ELIGIBLE_REGION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EligibleRegionPropertyIri> for &str {
	fn eq(&self, other: &EligibleRegionPropertyIri) -> bool {
		*self == ELIGIBLE_REGION_PROPERTY_IRI_HTTP || *self == ELIGIBLE_REGION_PROPERTY_IRI_HTTPS
	}
}
pub struct EligibleRegionPropertyIriOrLabel;
impl PartialEq<&str> for EligibleRegionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EligibleRegionPropertyIri || *other == ELIGIBLE_REGION_PROPERTY_LABEL
	}
}
impl PartialEq<EligibleRegionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EligibleRegionPropertyIriOrLabel) -> bool {
		*self == EligibleRegionPropertyIri || *self == ELIGIBLE_REGION_PROPERTY_LABEL
	}
}
