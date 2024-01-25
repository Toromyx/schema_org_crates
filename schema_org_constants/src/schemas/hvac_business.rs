/// <https://schema.org/HVACBusiness>
pub const HVAC_BUSINESS_IRI_HTTP: &str = "http://schema.org/HVACBusiness";
/// <https://schema.org/HVACBusiness>
pub const HVAC_BUSINESS_IRI_HTTPS: &str = "https://schema.org/HVACBusiness";
/// <https://schema.org/HVACBusiness>
pub const HVAC_BUSINESS_LABEL: &str = "HVACBusiness";
pub struct HvacBusinessIri;
impl PartialEq<&str> for HvacBusinessIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HVAC_BUSINESS_IRI_HTTP || *other == HVAC_BUSINESS_IRI_HTTPS
	}
}
impl PartialEq<HvacBusinessIri> for &str {
	fn eq(&self, other: &HvacBusinessIri) -> bool {
		*self == HVAC_BUSINESS_IRI_HTTP || *self == HVAC_BUSINESS_IRI_HTTPS
	}
}
pub struct HvacBusinessIriOrLabel;
impl PartialEq<&str> for HvacBusinessIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HvacBusinessIri || *other == HVAC_BUSINESS_LABEL
	}
}
impl PartialEq<HvacBusinessIriOrLabel> for &str {
	fn eq(&self, other: &HvacBusinessIriOrLabel) -> bool {
		*self == HvacBusinessIri || *self == HVAC_BUSINESS_LABEL
	}
}
