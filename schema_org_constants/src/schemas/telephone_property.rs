/// <https://schema.org/telephone>
pub const TELEPHONE_PROPERTY_IRI_HTTP: &str = "http://schema.org/telephone";
/// <https://schema.org/telephone>
pub const TELEPHONE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/telephone";
/// <https://schema.org/telephone>
pub const TELEPHONE_PROPERTY_LABEL: &str = "telephone";
pub struct TelephonePropertyIri;
impl PartialEq<&str> for TelephonePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TELEPHONE_PROPERTY_IRI_HTTP || *other == TELEPHONE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TelephonePropertyIri> for &str {
	fn eq(&self, other: &TelephonePropertyIri) -> bool {
		*self == TELEPHONE_PROPERTY_IRI_HTTP || *self == TELEPHONE_PROPERTY_IRI_HTTPS
	}
}
pub struct TelephonePropertyIriOrLabel;
impl PartialEq<&str> for TelephonePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TelephonePropertyIri || *other == TELEPHONE_PROPERTY_LABEL
	}
}
impl PartialEq<TelephonePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TelephonePropertyIriOrLabel) -> bool {
		*self == TelephonePropertyIri || *self == TELEPHONE_PROPERTY_LABEL
	}
}
