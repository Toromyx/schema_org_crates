/// <https://schema.org/SingleCenterTrial>
pub const SINGLE_CENTER_TRIAL_IRI_HTTP: &str = "http://schema.org/SingleCenterTrial";
/// <https://schema.org/SingleCenterTrial>
pub const SINGLE_CENTER_TRIAL_IRI_HTTPS: &str = "https://schema.org/SingleCenterTrial";
/// <https://schema.org/SingleCenterTrial>
pub const SINGLE_CENTER_TRIAL_LABEL: &str = "SingleCenterTrial";
pub struct SingleCenterTrialIri;
impl PartialEq<&str> for SingleCenterTrialIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SINGLE_CENTER_TRIAL_IRI_HTTP || *other == SINGLE_CENTER_TRIAL_IRI_HTTPS
	}
}
impl PartialEq<SingleCenterTrialIri> for &str {
	fn eq(&self, other: &SingleCenterTrialIri) -> bool {
		*self == SINGLE_CENTER_TRIAL_IRI_HTTP || *self == SINGLE_CENTER_TRIAL_IRI_HTTPS
	}
}
pub struct SingleCenterTrialIriOrLabel;
impl PartialEq<&str> for SingleCenterTrialIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SingleCenterTrialIri || *other == SINGLE_CENTER_TRIAL_LABEL
	}
}
impl PartialEq<SingleCenterTrialIriOrLabel> for &str {
	fn eq(&self, other: &SingleCenterTrialIriOrLabel) -> bool {
		*self == SingleCenterTrialIri || *self == SINGLE_CENTER_TRIAL_LABEL
	}
}
