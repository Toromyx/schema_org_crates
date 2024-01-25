/// <https://schema.org/AutomatedTeller>
pub const AUTOMATED_TELLER_IRI_HTTP: &str = "http://schema.org/AutomatedTeller";
/// <https://schema.org/AutomatedTeller>
pub const AUTOMATED_TELLER_IRI_HTTPS: &str = "https://schema.org/AutomatedTeller";
/// <https://schema.org/AutomatedTeller>
pub const AUTOMATED_TELLER_LABEL: &str = "AutomatedTeller";
pub struct AutomatedTellerIri;
impl PartialEq<&str> for AutomatedTellerIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUTOMATED_TELLER_IRI_HTTP || *other == AUTOMATED_TELLER_IRI_HTTPS
	}
}
impl PartialEq<AutomatedTellerIri> for &str {
	fn eq(&self, other: &AutomatedTellerIri) -> bool {
		*self == AUTOMATED_TELLER_IRI_HTTP || *self == AUTOMATED_TELLER_IRI_HTTPS
	}
}
pub struct AutomatedTellerIriOrLabel;
impl PartialEq<&str> for AutomatedTellerIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AutomatedTellerIri || *other == AUTOMATED_TELLER_LABEL
	}
}
impl PartialEq<AutomatedTellerIriOrLabel> for &str {
	fn eq(&self, other: &AutomatedTellerIriOrLabel) -> bool {
		*self == AutomatedTellerIri || *self == AUTOMATED_TELLER_LABEL
	}
}
