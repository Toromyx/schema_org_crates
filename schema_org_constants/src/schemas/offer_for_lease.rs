/// <https://schema.org/OfferForLease>
pub const OFFER_FOR_LEASE_IRI_HTTP: &str = "http://schema.org/OfferForLease";
/// <https://schema.org/OfferForLease>
pub const OFFER_FOR_LEASE_IRI_HTTPS: &str = "https://schema.org/OfferForLease";
/// <https://schema.org/OfferForLease>
pub const OFFER_FOR_LEASE_LABEL: &str = "OfferForLease";
pub struct OfferForLeaseIri;
impl PartialEq<&str> for OfferForLeaseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFER_FOR_LEASE_IRI_HTTP || *other == OFFER_FOR_LEASE_IRI_HTTPS
	}
}
impl PartialEq<OfferForLeaseIri> for &str {
	fn eq(&self, other: &OfferForLeaseIri) -> bool {
		*self == OFFER_FOR_LEASE_IRI_HTTP || *self == OFFER_FOR_LEASE_IRI_HTTPS
	}
}
pub struct OfferForLeaseIriOrLabel;
impl PartialEq<&str> for OfferForLeaseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OfferForLeaseIri || *other == OFFER_FOR_LEASE_LABEL
	}
}
impl PartialEq<OfferForLeaseIriOrLabel> for &str {
	fn eq(&self, other: &OfferForLeaseIriOrLabel) -> bool {
		*self == OfferForLeaseIri || *self == OFFER_FOR_LEASE_LABEL
	}
}
