/// <https://schema.org/PlaceboControlledTrial>
pub const PLACEBO_CONTROLLED_TRIAL_IRI_HTTP: &str = "http://schema.org/PlaceboControlledTrial";
/// <https://schema.org/PlaceboControlledTrial>
pub const PLACEBO_CONTROLLED_TRIAL_IRI_HTTPS: &str = "https://schema.org/PlaceboControlledTrial";
/// <https://schema.org/PlaceboControlledTrial>
pub const PLACEBO_CONTROLLED_TRIAL_LABEL: &str = "PlaceboControlledTrial";
pub struct PlaceboControlledTrialIri;
impl PartialEq<&str> for PlaceboControlledTrialIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PLACEBO_CONTROLLED_TRIAL_IRI_HTTP || *other == PLACEBO_CONTROLLED_TRIAL_IRI_HTTPS
	}
}
impl PartialEq<PlaceboControlledTrialIri> for &str {
	fn eq(&self, other: &PlaceboControlledTrialIri) -> bool {
		*self == PLACEBO_CONTROLLED_TRIAL_IRI_HTTP || *self == PLACEBO_CONTROLLED_TRIAL_IRI_HTTPS
	}
}
pub struct PlaceboControlledTrialIriOrLabel;
impl PartialEq<&str> for PlaceboControlledTrialIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PlaceboControlledTrialIri || *other == PLACEBO_CONTROLLED_TRIAL_LABEL
	}
}
impl PartialEq<PlaceboControlledTrialIriOrLabel> for &str {
	fn eq(&self, other: &PlaceboControlledTrialIriOrLabel) -> bool {
		*self == PlaceboControlledTrialIri || *self == PLACEBO_CONTROLLED_TRIAL_LABEL
	}
}
