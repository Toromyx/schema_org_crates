/// <https://schema.org/Demand>
pub const DEMAND_IRI_HTTP: &str = "http://schema.org/Demand";
/// <https://schema.org/Demand>
pub const DEMAND_IRI_HTTPS: &str = "https://schema.org/Demand";
/// <https://schema.org/Demand>
pub const DEMAND_LABEL: &str = "Demand";
pub struct DemandIri;
impl PartialEq<&str> for DemandIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEMAND_IRI_HTTP || *other == DEMAND_IRI_HTTPS
	}
}
impl PartialEq<DemandIri> for &str {
	fn eq(&self, other: &DemandIri) -> bool {
		*self == DEMAND_IRI_HTTP || *self == DEMAND_IRI_HTTPS
	}
}
pub struct DemandIriOrLabel;
impl PartialEq<&str> for DemandIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DemandIri || *other == DEMAND_LABEL
	}
}
impl PartialEq<DemandIriOrLabel> for &str {
	fn eq(&self, other: &DemandIriOrLabel) -> bool {
		*self == DemandIri || *self == DEMAND_LABEL
	}
}
