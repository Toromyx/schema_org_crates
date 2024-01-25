/// <https://schema.org/LowLactoseDiet>
pub const LOW_LACTOSE_DIET_IRI_HTTP: &str = "http://schema.org/LowLactoseDiet";
/// <https://schema.org/LowLactoseDiet>
pub const LOW_LACTOSE_DIET_IRI_HTTPS: &str = "https://schema.org/LowLactoseDiet";
/// <https://schema.org/LowLactoseDiet>
pub const LOW_LACTOSE_DIET_LABEL: &str = "LowLactoseDiet";
pub struct LowLactoseDietIri;
impl PartialEq<&str> for LowLactoseDietIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOW_LACTOSE_DIET_IRI_HTTP || *other == LOW_LACTOSE_DIET_IRI_HTTPS
	}
}
impl PartialEq<LowLactoseDietIri> for &str {
	fn eq(&self, other: &LowLactoseDietIri) -> bool {
		*self == LOW_LACTOSE_DIET_IRI_HTTP || *self == LOW_LACTOSE_DIET_IRI_HTTPS
	}
}
pub struct LowLactoseDietIriOrLabel;
impl PartialEq<&str> for LowLactoseDietIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LowLactoseDietIri || *other == LOW_LACTOSE_DIET_LABEL
	}
}
impl PartialEq<LowLactoseDietIriOrLabel> for &str {
	fn eq(&self, other: &LowLactoseDietIriOrLabel) -> bool {
		*self == LowLactoseDietIri || *self == LOW_LACTOSE_DIET_LABEL
	}
}
