/// <https://schema.org/InternationalTrial>
pub const INTERNATIONAL_TRIAL_IRI_HTTP: &str = "http://schema.org/InternationalTrial";
/// <https://schema.org/InternationalTrial>
pub const INTERNATIONAL_TRIAL_IRI_HTTPS: &str = "https://schema.org/InternationalTrial";
/// <https://schema.org/InternationalTrial>
pub const INTERNATIONAL_TRIAL_LABEL: &str = "InternationalTrial";
pub struct InternationalTrialIri;
impl PartialEq<&str> for InternationalTrialIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTERNATIONAL_TRIAL_IRI_HTTP || *other == INTERNATIONAL_TRIAL_IRI_HTTPS
	}
}
impl PartialEq<InternationalTrialIri> for &str {
	fn eq(&self, other: &InternationalTrialIri) -> bool {
		*self == INTERNATIONAL_TRIAL_IRI_HTTP || *self == INTERNATIONAL_TRIAL_IRI_HTTPS
	}
}
pub struct InternationalTrialIriOrLabel;
impl PartialEq<&str> for InternationalTrialIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InternationalTrialIri || *other == INTERNATIONAL_TRIAL_LABEL
	}
}
impl PartialEq<InternationalTrialIriOrLabel> for &str {
	fn eq(&self, other: &InternationalTrialIriOrLabel) -> bool {
		*self == InternationalTrialIri || *self == INTERNATIONAL_TRIAL_LABEL
	}
}
