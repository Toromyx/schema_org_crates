/// <https://schema.org/claimInterpreter>
pub const CLAIM_INTERPRETER_PROPERTY_IRI_HTTP: &str = "http://schema.org/claimInterpreter";
/// <https://schema.org/claimInterpreter>
pub const CLAIM_INTERPRETER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/claimInterpreter";
/// <https://schema.org/claimInterpreter>
pub const CLAIM_INTERPRETER_PROPERTY_LABEL: &str = "claimInterpreter";
pub struct ClaimInterpreterPropertyIri;
impl PartialEq<&str> for ClaimInterpreterPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CLAIM_INTERPRETER_PROPERTY_IRI_HTTP
			|| *other == CLAIM_INTERPRETER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ClaimInterpreterPropertyIri> for &str {
	fn eq(&self, other: &ClaimInterpreterPropertyIri) -> bool {
		*self == CLAIM_INTERPRETER_PROPERTY_IRI_HTTP
			|| *self == CLAIM_INTERPRETER_PROPERTY_IRI_HTTPS
	}
}
pub struct ClaimInterpreterPropertyIriOrLabel;
impl PartialEq<&str> for ClaimInterpreterPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ClaimInterpreterPropertyIri || *other == CLAIM_INTERPRETER_PROPERTY_LABEL
	}
}
impl PartialEq<ClaimInterpreterPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ClaimInterpreterPropertyIriOrLabel) -> bool {
		*self == ClaimInterpreterPropertyIri || *self == CLAIM_INTERPRETER_PROPERTY_LABEL
	}
}
