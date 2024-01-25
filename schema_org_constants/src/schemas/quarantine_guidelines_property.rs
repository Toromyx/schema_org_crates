/// <https://schema.org/quarantineGuidelines>
pub const QUARANTINE_GUIDELINES_PROPERTY_IRI_HTTP: &str = "http://schema.org/quarantineGuidelines";
/// <https://schema.org/quarantineGuidelines>
pub const QUARANTINE_GUIDELINES_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/quarantineGuidelines";
/// <https://schema.org/quarantineGuidelines>
pub const QUARANTINE_GUIDELINES_PROPERTY_LABEL: &str = "quarantineGuidelines";
pub struct QuarantineGuidelinesPropertyIri;
impl PartialEq<&str> for QuarantineGuidelinesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == QUARANTINE_GUIDELINES_PROPERTY_IRI_HTTP
			|| *other == QUARANTINE_GUIDELINES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<QuarantineGuidelinesPropertyIri> for &str {
	fn eq(&self, other: &QuarantineGuidelinesPropertyIri) -> bool {
		*self == QUARANTINE_GUIDELINES_PROPERTY_IRI_HTTP
			|| *self == QUARANTINE_GUIDELINES_PROPERTY_IRI_HTTPS
	}
}
pub struct QuarantineGuidelinesPropertyIriOrLabel;
impl PartialEq<&str> for QuarantineGuidelinesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == QuarantineGuidelinesPropertyIri || *other == QUARANTINE_GUIDELINES_PROPERTY_LABEL
	}
}
impl PartialEq<QuarantineGuidelinesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &QuarantineGuidelinesPropertyIriOrLabel) -> bool {
		*self == QuarantineGuidelinesPropertyIri || *self == QUARANTINE_GUIDELINES_PROPERTY_LABEL
	}
}
