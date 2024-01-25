/// <https://schema.org/isPlanForApartment>
pub const IS_PLAN_FOR_APARTMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/isPlanForApartment";
/// <https://schema.org/isPlanForApartment>
pub const IS_PLAN_FOR_APARTMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isPlanForApartment";
/// <https://schema.org/isPlanForApartment>
pub const IS_PLAN_FOR_APARTMENT_PROPERTY_LABEL: &str = "isPlanForApartment";
pub struct IsPlanForApartmentPropertyIri;
impl PartialEq<&str> for IsPlanForApartmentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_PLAN_FOR_APARTMENT_PROPERTY_IRI_HTTP
			|| *other == IS_PLAN_FOR_APARTMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsPlanForApartmentPropertyIri> for &str {
	fn eq(&self, other: &IsPlanForApartmentPropertyIri) -> bool {
		*self == IS_PLAN_FOR_APARTMENT_PROPERTY_IRI_HTTP
			|| *self == IS_PLAN_FOR_APARTMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct IsPlanForApartmentPropertyIriOrLabel;
impl PartialEq<&str> for IsPlanForApartmentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsPlanForApartmentPropertyIri || *other == IS_PLAN_FOR_APARTMENT_PROPERTY_LABEL
	}
}
impl PartialEq<IsPlanForApartmentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsPlanForApartmentPropertyIriOrLabel) -> bool {
		*self == IsPlanForApartmentPropertyIri || *self == IS_PLAN_FOR_APARTMENT_PROPERTY_LABEL
	}
}
