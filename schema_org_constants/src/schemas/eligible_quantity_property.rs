/// <https://schema.org/eligibleQuantity>
pub const ELIGIBLE_QUANTITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/eligibleQuantity";
/// <https://schema.org/eligibleQuantity>
pub const ELIGIBLE_QUANTITY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/eligibleQuantity";
/// <https://schema.org/eligibleQuantity>
pub const ELIGIBLE_QUANTITY_PROPERTY_LABEL: &str = "eligibleQuantity";
pub struct EligibleQuantityPropertyIri;
impl PartialEq<&str> for EligibleQuantityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ELIGIBLE_QUANTITY_PROPERTY_IRI_HTTP
			|| *other == ELIGIBLE_QUANTITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EligibleQuantityPropertyIri> for &str {
	fn eq(&self, other: &EligibleQuantityPropertyIri) -> bool {
		*self == ELIGIBLE_QUANTITY_PROPERTY_IRI_HTTP
			|| *self == ELIGIBLE_QUANTITY_PROPERTY_IRI_HTTPS
	}
}
pub struct EligibleQuantityPropertyIriOrLabel;
impl PartialEq<&str> for EligibleQuantityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EligibleQuantityPropertyIri || *other == ELIGIBLE_QUANTITY_PROPERTY_LABEL
	}
}
impl PartialEq<EligibleQuantityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EligibleQuantityPropertyIriOrLabel) -> bool {
		*self == EligibleQuantityPropertyIri || *self == ELIGIBLE_QUANTITY_PROPERTY_LABEL
	}
}
