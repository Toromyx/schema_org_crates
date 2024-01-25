/// <https://schema.org/DoubleBlindedTrial>
pub const DOUBLE_BLINDED_TRIAL_IRI_HTTP: &str = "http://schema.org/DoubleBlindedTrial";
/// <https://schema.org/DoubleBlindedTrial>
pub const DOUBLE_BLINDED_TRIAL_IRI_HTTPS: &str = "https://schema.org/DoubleBlindedTrial";
/// <https://schema.org/DoubleBlindedTrial>
pub const DOUBLE_BLINDED_TRIAL_LABEL: &str = "DoubleBlindedTrial";
pub struct DoubleBlindedTrialIri;
impl PartialEq<&str> for DoubleBlindedTrialIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOUBLE_BLINDED_TRIAL_IRI_HTTP || *other == DOUBLE_BLINDED_TRIAL_IRI_HTTPS
	}
}
impl PartialEq<DoubleBlindedTrialIri> for &str {
	fn eq(&self, other: &DoubleBlindedTrialIri) -> bool {
		*self == DOUBLE_BLINDED_TRIAL_IRI_HTTP || *self == DOUBLE_BLINDED_TRIAL_IRI_HTTPS
	}
}
pub struct DoubleBlindedTrialIriOrLabel;
impl PartialEq<&str> for DoubleBlindedTrialIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DoubleBlindedTrialIri || *other == DOUBLE_BLINDED_TRIAL_LABEL
	}
}
impl PartialEq<DoubleBlindedTrialIriOrLabel> for &str {
	fn eq(&self, other: &DoubleBlindedTrialIriOrLabel) -> bool {
		*self == DoubleBlindedTrialIri || *self == DOUBLE_BLINDED_TRIAL_LABEL
	}
}
