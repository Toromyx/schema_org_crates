/// <https://schema.org/DistanceFee>
pub const DISTANCE_FEE_IRI_HTTP: &str = "http://schema.org/DistanceFee";
/// <https://schema.org/DistanceFee>
pub const DISTANCE_FEE_IRI_HTTPS: &str = "https://schema.org/DistanceFee";
/// <https://schema.org/DistanceFee>
pub const DISTANCE_FEE_LABEL: &str = "DistanceFee";
pub struct DistanceFeeIri;
impl PartialEq<&str> for DistanceFeeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISTANCE_FEE_IRI_HTTP || *other == DISTANCE_FEE_IRI_HTTPS
	}
}
impl PartialEq<DistanceFeeIri> for &str {
	fn eq(&self, other: &DistanceFeeIri) -> bool {
		*self == DISTANCE_FEE_IRI_HTTP || *self == DISTANCE_FEE_IRI_HTTPS
	}
}
pub struct DistanceFeeIriOrLabel;
impl PartialEq<&str> for DistanceFeeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DistanceFeeIri || *other == DISTANCE_FEE_LABEL
	}
}
impl PartialEq<DistanceFeeIriOrLabel> for &str {
	fn eq(&self, other: &DistanceFeeIriOrLabel) -> bool {
		*self == DistanceFeeIri || *self == DISTANCE_FEE_LABEL
	}
}
