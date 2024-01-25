/// <https://schema.org/OpenTrial>
pub const OPEN_TRIAL_IRI_HTTP: &str = "http://schema.org/OpenTrial";
/// <https://schema.org/OpenTrial>
pub const OPEN_TRIAL_IRI_HTTPS: &str = "https://schema.org/OpenTrial";
/// <https://schema.org/OpenTrial>
pub const OPEN_TRIAL_LABEL: &str = "OpenTrial";
pub struct OpenTrialIri;
impl PartialEq<&str> for OpenTrialIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OPEN_TRIAL_IRI_HTTP || *other == OPEN_TRIAL_IRI_HTTPS
	}
}
impl PartialEq<OpenTrialIri> for &str {
	fn eq(&self, other: &OpenTrialIri) -> bool {
		*self == OPEN_TRIAL_IRI_HTTP || *self == OPEN_TRIAL_IRI_HTTPS
	}
}
pub struct OpenTrialIriOrLabel;
impl PartialEq<&str> for OpenTrialIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OpenTrialIri || *other == OPEN_TRIAL_LABEL
	}
}
impl PartialEq<OpenTrialIriOrLabel> for &str {
	fn eq(&self, other: &OpenTrialIriOrLabel) -> bool {
		*self == OpenTrialIri || *self == OPEN_TRIAL_LABEL
	}
}
