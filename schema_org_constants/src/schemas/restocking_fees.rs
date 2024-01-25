/// <https://schema.org/RestockingFees>
pub const RESTOCKING_FEES_IRI_HTTP: &str = "http://schema.org/RestockingFees";
/// <https://schema.org/RestockingFees>
pub const RESTOCKING_FEES_IRI_HTTPS: &str = "https://schema.org/RestockingFees";
/// <https://schema.org/RestockingFees>
pub const RESTOCKING_FEES_LABEL: &str = "RestockingFees";
pub struct RestockingFeesIri;
impl PartialEq<&str> for RestockingFeesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESTOCKING_FEES_IRI_HTTP || *other == RESTOCKING_FEES_IRI_HTTPS
	}
}
impl PartialEq<RestockingFeesIri> for &str {
	fn eq(&self, other: &RestockingFeesIri) -> bool {
		*self == RESTOCKING_FEES_IRI_HTTP || *self == RESTOCKING_FEES_IRI_HTTPS
	}
}
pub struct RestockingFeesIriOrLabel;
impl PartialEq<&str> for RestockingFeesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RestockingFeesIri || *other == RESTOCKING_FEES_LABEL
	}
}
impl PartialEq<RestockingFeesIriOrLabel> for &str {
	fn eq(&self, other: &RestockingFeesIriOrLabel) -> bool {
		*self == RestockingFeesIri || *self == RESTOCKING_FEES_LABEL
	}
}
