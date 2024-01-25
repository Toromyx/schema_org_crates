/// <https://schema.org/MultiCenterTrial>
pub const MULTI_CENTER_TRIAL_IRI_HTTP: &str = "http://schema.org/MultiCenterTrial";
/// <https://schema.org/MultiCenterTrial>
pub const MULTI_CENTER_TRIAL_IRI_HTTPS: &str = "https://schema.org/MultiCenterTrial";
/// <https://schema.org/MultiCenterTrial>
pub const MULTI_CENTER_TRIAL_LABEL: &str = "MultiCenterTrial";
pub struct MultiCenterTrialIri;
impl PartialEq<&str> for MultiCenterTrialIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MULTI_CENTER_TRIAL_IRI_HTTP || *other == MULTI_CENTER_TRIAL_IRI_HTTPS
	}
}
impl PartialEq<MultiCenterTrialIri> for &str {
	fn eq(&self, other: &MultiCenterTrialIri) -> bool {
		*self == MULTI_CENTER_TRIAL_IRI_HTTP || *self == MULTI_CENTER_TRIAL_IRI_HTTPS
	}
}
pub struct MultiCenterTrialIriOrLabel;
impl PartialEq<&str> for MultiCenterTrialIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MultiCenterTrialIri || *other == MULTI_CENTER_TRIAL_LABEL
	}
}
impl PartialEq<MultiCenterTrialIriOrLabel> for &str {
	fn eq(&self, other: &MultiCenterTrialIriOrLabel) -> bool {
		*self == MultiCenterTrialIri || *self == MULTI_CENTER_TRIAL_LABEL
	}
}
