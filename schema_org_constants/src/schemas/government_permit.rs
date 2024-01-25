/// <https://schema.org/GovernmentPermit>
pub const GOVERNMENT_PERMIT_IRI_HTTP: &str = "http://schema.org/GovernmentPermit";
/// <https://schema.org/GovernmentPermit>
pub const GOVERNMENT_PERMIT_IRI_HTTPS: &str = "https://schema.org/GovernmentPermit";
/// <https://schema.org/GovernmentPermit>
pub const GOVERNMENT_PERMIT_LABEL: &str = "GovernmentPermit";
pub struct GovernmentPermitIri;
impl PartialEq<&str> for GovernmentPermitIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GOVERNMENT_PERMIT_IRI_HTTP || *other == GOVERNMENT_PERMIT_IRI_HTTPS
	}
}
impl PartialEq<GovernmentPermitIri> for &str {
	fn eq(&self, other: &GovernmentPermitIri) -> bool {
		*self == GOVERNMENT_PERMIT_IRI_HTTP || *self == GOVERNMENT_PERMIT_IRI_HTTPS
	}
}
pub struct GovernmentPermitIriOrLabel;
impl PartialEq<&str> for GovernmentPermitIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GovernmentPermitIri || *other == GOVERNMENT_PERMIT_LABEL
	}
}
impl PartialEq<GovernmentPermitIriOrLabel> for &str {
	fn eq(&self, other: &GovernmentPermitIriOrLabel) -> bool {
		*self == GovernmentPermitIri || *self == GOVERNMENT_PERMIT_LABEL
	}
}
