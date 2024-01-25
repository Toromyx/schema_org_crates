/// <https://schema.org/RandomizedTrial>
pub const RANDOMIZED_TRIAL_IRI_HTTP: &str = "http://schema.org/RandomizedTrial";
/// <https://schema.org/RandomizedTrial>
pub const RANDOMIZED_TRIAL_IRI_HTTPS: &str = "https://schema.org/RandomizedTrial";
/// <https://schema.org/RandomizedTrial>
pub const RANDOMIZED_TRIAL_LABEL: &str = "RandomizedTrial";
pub struct RandomizedTrialIri;
impl PartialEq<&str> for RandomizedTrialIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RANDOMIZED_TRIAL_IRI_HTTP || *other == RANDOMIZED_TRIAL_IRI_HTTPS
	}
}
impl PartialEq<RandomizedTrialIri> for &str {
	fn eq(&self, other: &RandomizedTrialIri) -> bool {
		*self == RANDOMIZED_TRIAL_IRI_HTTP || *self == RANDOMIZED_TRIAL_IRI_HTTPS
	}
}
pub struct RandomizedTrialIriOrLabel;
impl PartialEq<&str> for RandomizedTrialIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RandomizedTrialIri || *other == RANDOMIZED_TRIAL_LABEL
	}
}
impl PartialEq<RandomizedTrialIriOrLabel> for &str {
	fn eq(&self, other: &RandomizedTrialIriOrLabel) -> bool {
		*self == RandomizedTrialIri || *self == RANDOMIZED_TRIAL_LABEL
	}
}
