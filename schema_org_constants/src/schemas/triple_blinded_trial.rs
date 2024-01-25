/// <https://schema.org/TripleBlindedTrial>
pub const TRIPLE_BLINDED_TRIAL_IRI_HTTP: &str = "http://schema.org/TripleBlindedTrial";
/// <https://schema.org/TripleBlindedTrial>
pub const TRIPLE_BLINDED_TRIAL_IRI_HTTPS: &str = "https://schema.org/TripleBlindedTrial";
/// <https://schema.org/TripleBlindedTrial>
pub const TRIPLE_BLINDED_TRIAL_LABEL: &str = "TripleBlindedTrial";
pub struct TripleBlindedTrialIri;
impl PartialEq<&str> for TripleBlindedTrialIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRIPLE_BLINDED_TRIAL_IRI_HTTP || *other == TRIPLE_BLINDED_TRIAL_IRI_HTTPS
	}
}
impl PartialEq<TripleBlindedTrialIri> for &str {
	fn eq(&self, other: &TripleBlindedTrialIri) -> bool {
		*self == TRIPLE_BLINDED_TRIAL_IRI_HTTP || *self == TRIPLE_BLINDED_TRIAL_IRI_HTTPS
	}
}
pub struct TripleBlindedTrialIriOrLabel;
impl PartialEq<&str> for TripleBlindedTrialIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TripleBlindedTrialIri || *other == TRIPLE_BLINDED_TRIAL_LABEL
	}
}
impl PartialEq<TripleBlindedTrialIriOrLabel> for &str {
	fn eq(&self, other: &TripleBlindedTrialIriOrLabel) -> bool {
		*self == TripleBlindedTrialIri || *self == TRIPLE_BLINDED_TRIAL_LABEL
	}
}
