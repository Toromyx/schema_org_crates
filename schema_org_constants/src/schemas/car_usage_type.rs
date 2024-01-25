/// <https://schema.org/CarUsageType>
pub const CAR_USAGE_TYPE_IRI_HTTP: &str = "http://schema.org/CarUsageType";
/// <https://schema.org/CarUsageType>
pub const CAR_USAGE_TYPE_IRI_HTTPS: &str = "https://schema.org/CarUsageType";
/// <https://schema.org/CarUsageType>
pub const CAR_USAGE_TYPE_LABEL: &str = "CarUsageType";
pub struct CarUsageTypeIri;
impl PartialEq<&str> for CarUsageTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CAR_USAGE_TYPE_IRI_HTTP || *other == CAR_USAGE_TYPE_IRI_HTTPS
	}
}
impl PartialEq<CarUsageTypeIri> for &str {
	fn eq(&self, other: &CarUsageTypeIri) -> bool {
		*self == CAR_USAGE_TYPE_IRI_HTTP || *self == CAR_USAGE_TYPE_IRI_HTTPS
	}
}
pub struct CarUsageTypeIriOrLabel;
impl PartialEq<&str> for CarUsageTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CarUsageTypeIri || *other == CAR_USAGE_TYPE_LABEL
	}
}
impl PartialEq<CarUsageTypeIriOrLabel> for &str {
	fn eq(&self, other: &CarUsageTypeIriOrLabel) -> bool {
		*self == CarUsageTypeIri || *self == CAR_USAGE_TYPE_LABEL
	}
}
