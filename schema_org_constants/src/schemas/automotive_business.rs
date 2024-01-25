/// <https://schema.org/AutomotiveBusiness>
pub const AUTOMOTIVE_BUSINESS_IRI_HTTP: &str = "http://schema.org/AutomotiveBusiness";
/// <https://schema.org/AutomotiveBusiness>
pub const AUTOMOTIVE_BUSINESS_IRI_HTTPS: &str = "https://schema.org/AutomotiveBusiness";
/// <https://schema.org/AutomotiveBusiness>
pub const AUTOMOTIVE_BUSINESS_LABEL: &str = "AutomotiveBusiness";
pub struct AutomotiveBusinessIri;
impl PartialEq<&str> for AutomotiveBusinessIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUTOMOTIVE_BUSINESS_IRI_HTTP || *other == AUTOMOTIVE_BUSINESS_IRI_HTTPS
	}
}
impl PartialEq<AutomotiveBusinessIri> for &str {
	fn eq(&self, other: &AutomotiveBusinessIri) -> bool {
		*self == AUTOMOTIVE_BUSINESS_IRI_HTTP || *self == AUTOMOTIVE_BUSINESS_IRI_HTTPS
	}
}
pub struct AutomotiveBusinessIriOrLabel;
impl PartialEq<&str> for AutomotiveBusinessIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AutomotiveBusinessIri || *other == AUTOMOTIVE_BUSINESS_LABEL
	}
}
impl PartialEq<AutomotiveBusinessIriOrLabel> for &str {
	fn eq(&self, other: &AutomotiveBusinessIriOrLabel) -> bool {
		*self == AutomotiveBusinessIri || *self == AUTOMOTIVE_BUSINESS_LABEL
	}
}
