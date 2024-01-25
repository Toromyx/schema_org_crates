/// <https://schema.org/carrierRequirements>
pub const CARRIER_REQUIREMENTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/carrierRequirements";
/// <https://schema.org/carrierRequirements>
pub const CARRIER_REQUIREMENTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/carrierRequirements";
/// <https://schema.org/carrierRequirements>
pub const CARRIER_REQUIREMENTS_PROPERTY_LABEL: &str = "carrierRequirements";
pub struct CarrierRequirementsPropertyIri;
impl PartialEq<&str> for CarrierRequirementsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CARRIER_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *other == CARRIER_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CarrierRequirementsPropertyIri> for &str {
	fn eq(&self, other: &CarrierRequirementsPropertyIri) -> bool {
		*self == CARRIER_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *self == CARRIER_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct CarrierRequirementsPropertyIriOrLabel;
impl PartialEq<&str> for CarrierRequirementsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CarrierRequirementsPropertyIri || *other == CARRIER_REQUIREMENTS_PROPERTY_LABEL
	}
}
impl PartialEq<CarrierRequirementsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CarrierRequirementsPropertyIriOrLabel) -> bool {
		*self == CarrierRequirementsPropertyIri || *self == CARRIER_REQUIREMENTS_PROPERTY_LABEL
	}
}
