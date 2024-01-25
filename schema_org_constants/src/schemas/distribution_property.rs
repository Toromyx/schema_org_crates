/// <https://schema.org/distribution>
pub const DISTRIBUTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/distribution";
/// <https://schema.org/distribution>
pub const DISTRIBUTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/distribution";
/// <https://schema.org/distribution>
pub const DISTRIBUTION_PROPERTY_LABEL: &str = "distribution";
pub struct DistributionPropertyIri;
impl PartialEq<&str> for DistributionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISTRIBUTION_PROPERTY_IRI_HTTP || *other == DISTRIBUTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DistributionPropertyIri> for &str {
	fn eq(&self, other: &DistributionPropertyIri) -> bool {
		*self == DISTRIBUTION_PROPERTY_IRI_HTTP || *self == DISTRIBUTION_PROPERTY_IRI_HTTPS
	}
}
pub struct DistributionPropertyIriOrLabel;
impl PartialEq<&str> for DistributionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DistributionPropertyIri || *other == DISTRIBUTION_PROPERTY_LABEL
	}
}
impl PartialEq<DistributionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DistributionPropertyIriOrLabel) -> bool {
		*self == DistributionPropertyIri || *self == DISTRIBUTION_PROPERTY_LABEL
	}
}
