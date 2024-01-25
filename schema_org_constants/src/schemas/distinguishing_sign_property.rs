/// <https://schema.org/distinguishingSign>
pub const DISTINGUISHING_SIGN_PROPERTY_IRI_HTTP: &str = "http://schema.org/distinguishingSign";
/// <https://schema.org/distinguishingSign>
pub const DISTINGUISHING_SIGN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/distinguishingSign";
/// <https://schema.org/distinguishingSign>
pub const DISTINGUISHING_SIGN_PROPERTY_LABEL: &str = "distinguishingSign";
pub struct DistinguishingSignPropertyIri;
impl PartialEq<&str> for DistinguishingSignPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISTINGUISHING_SIGN_PROPERTY_IRI_HTTP
			|| *other == DISTINGUISHING_SIGN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DistinguishingSignPropertyIri> for &str {
	fn eq(&self, other: &DistinguishingSignPropertyIri) -> bool {
		*self == DISTINGUISHING_SIGN_PROPERTY_IRI_HTTP
			|| *self == DISTINGUISHING_SIGN_PROPERTY_IRI_HTTPS
	}
}
pub struct DistinguishingSignPropertyIriOrLabel;
impl PartialEq<&str> for DistinguishingSignPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DistinguishingSignPropertyIri || *other == DISTINGUISHING_SIGN_PROPERTY_LABEL
	}
}
impl PartialEq<DistinguishingSignPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DistinguishingSignPropertyIriOrLabel) -> bool {
		*self == DistinguishingSignPropertyIri || *self == DISTINGUISHING_SIGN_PROPERTY_LABEL
	}
}
