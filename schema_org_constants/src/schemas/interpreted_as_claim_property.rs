/// <https://schema.org/interpretedAsClaim>
pub const INTERPRETED_AS_CLAIM_PROPERTY_IRI_HTTP: &str = "http://schema.org/interpretedAsClaim";
/// <https://schema.org/interpretedAsClaim>
pub const INTERPRETED_AS_CLAIM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/interpretedAsClaim";
/// <https://schema.org/interpretedAsClaim>
pub const INTERPRETED_AS_CLAIM_PROPERTY_LABEL: &str = "interpretedAsClaim";
pub struct InterpretedAsClaimPropertyIri;
impl PartialEq<&str> for InterpretedAsClaimPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTERPRETED_AS_CLAIM_PROPERTY_IRI_HTTP
			|| *other == INTERPRETED_AS_CLAIM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InterpretedAsClaimPropertyIri> for &str {
	fn eq(&self, other: &InterpretedAsClaimPropertyIri) -> bool {
		*self == INTERPRETED_AS_CLAIM_PROPERTY_IRI_HTTP
			|| *self == INTERPRETED_AS_CLAIM_PROPERTY_IRI_HTTPS
	}
}
pub struct InterpretedAsClaimPropertyIriOrLabel;
impl PartialEq<&str> for InterpretedAsClaimPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InterpretedAsClaimPropertyIri || *other == INTERPRETED_AS_CLAIM_PROPERTY_LABEL
	}
}
impl PartialEq<InterpretedAsClaimPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InterpretedAsClaimPropertyIriOrLabel) -> bool {
		*self == InterpretedAsClaimPropertyIri || *self == INTERPRETED_AS_CLAIM_PROPERTY_LABEL
	}
}
