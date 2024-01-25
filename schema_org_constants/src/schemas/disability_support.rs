/// <https://schema.org/DisabilitySupport>
pub const DISABILITY_SUPPORT_IRI_HTTP: &str = "http://schema.org/DisabilitySupport";
/// <https://schema.org/DisabilitySupport>
pub const DISABILITY_SUPPORT_IRI_HTTPS: &str = "https://schema.org/DisabilitySupport";
/// <https://schema.org/DisabilitySupport>
pub const DISABILITY_SUPPORT_LABEL: &str = "DisabilitySupport";
pub struct DisabilitySupportIri;
impl PartialEq<&str> for DisabilitySupportIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISABILITY_SUPPORT_IRI_HTTP || *other == DISABILITY_SUPPORT_IRI_HTTPS
	}
}
impl PartialEq<DisabilitySupportIri> for &str {
	fn eq(&self, other: &DisabilitySupportIri) -> bool {
		*self == DISABILITY_SUPPORT_IRI_HTTP || *self == DISABILITY_SUPPORT_IRI_HTTPS
	}
}
pub struct DisabilitySupportIriOrLabel;
impl PartialEq<&str> for DisabilitySupportIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DisabilitySupportIri || *other == DISABILITY_SUPPORT_LABEL
	}
}
impl PartialEq<DisabilitySupportIriOrLabel> for &str {
	fn eq(&self, other: &DisabilitySupportIriOrLabel) -> bool {
		*self == DisabilitySupportIri || *self == DISABILITY_SUPPORT_LABEL
	}
}
