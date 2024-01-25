/// <https://schema.org/estimatedCost>
pub const ESTIMATED_COST_PROPERTY_IRI_HTTP: &str = "http://schema.org/estimatedCost";
/// <https://schema.org/estimatedCost>
pub const ESTIMATED_COST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/estimatedCost";
/// <https://schema.org/estimatedCost>
pub const ESTIMATED_COST_PROPERTY_LABEL: &str = "estimatedCost";
pub struct EstimatedCostPropertyIri;
impl PartialEq<&str> for EstimatedCostPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ESTIMATED_COST_PROPERTY_IRI_HTTP || *other == ESTIMATED_COST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EstimatedCostPropertyIri> for &str {
	fn eq(&self, other: &EstimatedCostPropertyIri) -> bool {
		*self == ESTIMATED_COST_PROPERTY_IRI_HTTP || *self == ESTIMATED_COST_PROPERTY_IRI_HTTPS
	}
}
pub struct EstimatedCostPropertyIriOrLabel;
impl PartialEq<&str> for EstimatedCostPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EstimatedCostPropertyIri || *other == ESTIMATED_COST_PROPERTY_LABEL
	}
}
impl PartialEq<EstimatedCostPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EstimatedCostPropertyIriOrLabel) -> bool {
		*self == EstimatedCostPropertyIri || *self == ESTIMATED_COST_PROPERTY_LABEL
	}
}
