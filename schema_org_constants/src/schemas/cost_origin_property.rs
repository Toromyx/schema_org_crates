/// <https://schema.org/costOrigin>
pub const COST_ORIGIN_PROPERTY_IRI_HTTP: &str = "http://schema.org/costOrigin";
/// <https://schema.org/costOrigin>
pub const COST_ORIGIN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/costOrigin";
/// <https://schema.org/costOrigin>
pub const COST_ORIGIN_PROPERTY_LABEL: &str = "costOrigin";
pub struct CostOriginPropertyIri;
impl PartialEq<&str> for CostOriginPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COST_ORIGIN_PROPERTY_IRI_HTTP || *other == COST_ORIGIN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CostOriginPropertyIri> for &str {
	fn eq(&self, other: &CostOriginPropertyIri) -> bool {
		*self == COST_ORIGIN_PROPERTY_IRI_HTTP || *self == COST_ORIGIN_PROPERTY_IRI_HTTPS
	}
}
pub struct CostOriginPropertyIriOrLabel;
impl PartialEq<&str> for CostOriginPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CostOriginPropertyIri || *other == COST_ORIGIN_PROPERTY_LABEL
	}
}
impl PartialEq<CostOriginPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CostOriginPropertyIriOrLabel) -> bool {
		*self == CostOriginPropertyIri || *self == COST_ORIGIN_PROPERTY_LABEL
	}
}
