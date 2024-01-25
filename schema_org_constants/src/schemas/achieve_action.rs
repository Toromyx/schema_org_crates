/// <https://schema.org/AchieveAction>
pub const ACHIEVE_ACTION_IRI_HTTP: &str = "http://schema.org/AchieveAction";
/// <https://schema.org/AchieveAction>
pub const ACHIEVE_ACTION_IRI_HTTPS: &str = "https://schema.org/AchieveAction";
/// <https://schema.org/AchieveAction>
pub const ACHIEVE_ACTION_LABEL: &str = "AchieveAction";
pub struct AchieveActionIri;
impl PartialEq<&str> for AchieveActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACHIEVE_ACTION_IRI_HTTP || *other == ACHIEVE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<AchieveActionIri> for &str {
	fn eq(&self, other: &AchieveActionIri) -> bool {
		*self == ACHIEVE_ACTION_IRI_HTTP || *self == ACHIEVE_ACTION_IRI_HTTPS
	}
}
pub struct AchieveActionIriOrLabel;
impl PartialEq<&str> for AchieveActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AchieveActionIri || *other == ACHIEVE_ACTION_LABEL
	}
}
impl PartialEq<AchieveActionIriOrLabel> for &str {
	fn eq(&self, other: &AchieveActionIriOrLabel) -> bool {
		*self == AchieveActionIri || *self == ACHIEVE_ACTION_LABEL
	}
}
