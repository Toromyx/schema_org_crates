/// <https://schema.org/trialDesign>
pub const TRIAL_DESIGN_PROPERTY_IRI_HTTP: &str = "http://schema.org/trialDesign";
/// <https://schema.org/trialDesign>
pub const TRIAL_DESIGN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/trialDesign";
/// <https://schema.org/trialDesign>
pub const TRIAL_DESIGN_PROPERTY_LABEL: &str = "trialDesign";
pub struct TrialDesignPropertyIri;
impl PartialEq<&str> for TrialDesignPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRIAL_DESIGN_PROPERTY_IRI_HTTP || *other == TRIAL_DESIGN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TrialDesignPropertyIri> for &str {
	fn eq(&self, other: &TrialDesignPropertyIri) -> bool {
		*self == TRIAL_DESIGN_PROPERTY_IRI_HTTP || *self == TRIAL_DESIGN_PROPERTY_IRI_HTTPS
	}
}
pub struct TrialDesignPropertyIriOrLabel;
impl PartialEq<&str> for TrialDesignPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrialDesignPropertyIri || *other == TRIAL_DESIGN_PROPERTY_LABEL
	}
}
impl PartialEq<TrialDesignPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TrialDesignPropertyIriOrLabel) -> bool {
		*self == TrialDesignPropertyIri || *self == TRIAL_DESIGN_PROPERTY_LABEL
	}
}
