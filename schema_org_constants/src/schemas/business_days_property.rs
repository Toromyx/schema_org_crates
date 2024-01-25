/// <https://schema.org/businessDays>
pub const BUSINESS_DAYS_PROPERTY_IRI_HTTP: &str = "http://schema.org/businessDays";
/// <https://schema.org/businessDays>
pub const BUSINESS_DAYS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/businessDays";
/// <https://schema.org/businessDays>
pub const BUSINESS_DAYS_PROPERTY_LABEL: &str = "businessDays";
pub struct BusinessDaysPropertyIri;
impl PartialEq<&str> for BusinessDaysPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUSINESS_DAYS_PROPERTY_IRI_HTTP || *other == BUSINESS_DAYS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BusinessDaysPropertyIri> for &str {
	fn eq(&self, other: &BusinessDaysPropertyIri) -> bool {
		*self == BUSINESS_DAYS_PROPERTY_IRI_HTTP || *self == BUSINESS_DAYS_PROPERTY_IRI_HTTPS
	}
}
pub struct BusinessDaysPropertyIriOrLabel;
impl PartialEq<&str> for BusinessDaysPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BusinessDaysPropertyIri || *other == BUSINESS_DAYS_PROPERTY_LABEL
	}
}
impl PartialEq<BusinessDaysPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BusinessDaysPropertyIriOrLabel) -> bool {
		*self == BusinessDaysPropertyIri || *self == BUSINESS_DAYS_PROPERTY_LABEL
	}
}
