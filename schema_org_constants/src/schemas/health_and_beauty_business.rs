/// <https://schema.org/HealthAndBeautyBusiness>
pub const HEALTH_AND_BEAUTY_BUSINESS_IRI_HTTP: &str = "http://schema.org/HealthAndBeautyBusiness";
/// <https://schema.org/HealthAndBeautyBusiness>
pub const HEALTH_AND_BEAUTY_BUSINESS_IRI_HTTPS: &str = "https://schema.org/HealthAndBeautyBusiness";
/// <https://schema.org/HealthAndBeautyBusiness>
pub const HEALTH_AND_BEAUTY_BUSINESS_LABEL: &str = "HealthAndBeautyBusiness";
pub struct HealthAndBeautyBusinessIri;
impl PartialEq<&str> for HealthAndBeautyBusinessIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_AND_BEAUTY_BUSINESS_IRI_HTTP
			|| *other == HEALTH_AND_BEAUTY_BUSINESS_IRI_HTTPS
	}
}
impl PartialEq<HealthAndBeautyBusinessIri> for &str {
	fn eq(&self, other: &HealthAndBeautyBusinessIri) -> bool {
		*self == HEALTH_AND_BEAUTY_BUSINESS_IRI_HTTP
			|| *self == HEALTH_AND_BEAUTY_BUSINESS_IRI_HTTPS
	}
}
pub struct HealthAndBeautyBusinessIriOrLabel;
impl PartialEq<&str> for HealthAndBeautyBusinessIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthAndBeautyBusinessIri || *other == HEALTH_AND_BEAUTY_BUSINESS_LABEL
	}
}
impl PartialEq<HealthAndBeautyBusinessIriOrLabel> for &str {
	fn eq(&self, other: &HealthAndBeautyBusinessIriOrLabel) -> bool {
		*self == HealthAndBeautyBusinessIri || *self == HEALTH_AND_BEAUTY_BUSINESS_LABEL
	}
}
