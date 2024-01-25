/// <https://schema.org/creditedTo>
pub const CREDITED_TO_PROPERTY_IRI_HTTP: &str = "http://schema.org/creditedTo";
/// <https://schema.org/creditedTo>
pub const CREDITED_TO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/creditedTo";
/// <https://schema.org/creditedTo>
pub const CREDITED_TO_PROPERTY_LABEL: &str = "creditedTo";
pub struct CreditedToPropertyIri;
impl PartialEq<&str> for CreditedToPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CREDITED_TO_PROPERTY_IRI_HTTP || *other == CREDITED_TO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CreditedToPropertyIri> for &str {
	fn eq(&self, other: &CreditedToPropertyIri) -> bool {
		*self == CREDITED_TO_PROPERTY_IRI_HTTP || *self == CREDITED_TO_PROPERTY_IRI_HTTPS
	}
}
pub struct CreditedToPropertyIriOrLabel;
impl PartialEq<&str> for CreditedToPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CreditedToPropertyIri || *other == CREDITED_TO_PROPERTY_LABEL
	}
}
impl PartialEq<CreditedToPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CreditedToPropertyIriOrLabel) -> bool {
		*self == CreditedToPropertyIri || *self == CREDITED_TO_PROPERTY_LABEL
	}
}
