/// <https://schema.org/lender>
pub const LENDER_PROPERTY_IRI_HTTP: &str = "http://schema.org/lender";
/// <https://schema.org/lender>
pub const LENDER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/lender";
/// <https://schema.org/lender>
pub const LENDER_PROPERTY_LABEL: &str = "lender";
pub struct LenderPropertyIri;
impl PartialEq<&str> for LenderPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LENDER_PROPERTY_IRI_HTTP || *other == LENDER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LenderPropertyIri> for &str {
	fn eq(&self, other: &LenderPropertyIri) -> bool {
		*self == LENDER_PROPERTY_IRI_HTTP || *self == LENDER_PROPERTY_IRI_HTTPS
	}
}
pub struct LenderPropertyIriOrLabel;
impl PartialEq<&str> for LenderPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LenderPropertyIri || *other == LENDER_PROPERTY_LABEL
	}
}
impl PartialEq<LenderPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LenderPropertyIriOrLabel) -> bool {
		*self == LenderPropertyIri || *self == LENDER_PROPERTY_LABEL
	}
}
