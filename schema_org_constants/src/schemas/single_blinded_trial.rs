/// <https://schema.org/SingleBlindedTrial>
pub const SINGLE_BLINDED_TRIAL_IRI_HTTP: &str = "http://schema.org/SingleBlindedTrial";
/// <https://schema.org/SingleBlindedTrial>
pub const SINGLE_BLINDED_TRIAL_IRI_HTTPS: &str = "https://schema.org/SingleBlindedTrial";
/// <https://schema.org/SingleBlindedTrial>
pub const SINGLE_BLINDED_TRIAL_LABEL: &str = "SingleBlindedTrial";
pub struct SingleBlindedTrialIri;
impl PartialEq<&str> for SingleBlindedTrialIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SINGLE_BLINDED_TRIAL_IRI_HTTP || *other == SINGLE_BLINDED_TRIAL_IRI_HTTPS
	}
}
impl PartialEq<SingleBlindedTrialIri> for &str {
	fn eq(&self, other: &SingleBlindedTrialIri) -> bool {
		*self == SINGLE_BLINDED_TRIAL_IRI_HTTP || *self == SINGLE_BLINDED_TRIAL_IRI_HTTPS
	}
}
pub struct SingleBlindedTrialIriOrLabel;
impl PartialEq<&str> for SingleBlindedTrialIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SingleBlindedTrialIri || *other == SINGLE_BLINDED_TRIAL_LABEL
	}
}
impl PartialEq<SingleBlindedTrialIriOrLabel> for &str {
	fn eq(&self, other: &SingleBlindedTrialIriOrLabel) -> bool {
		*self == SingleBlindedTrialIri || *self == SINGLE_BLINDED_TRIAL_LABEL
	}
}
