/// <https://schema.org/followup>
pub const FOLLOWUP_PROPERTY_IRI_HTTP: &str = "http://schema.org/followup";
/// <https://schema.org/followup>
pub const FOLLOWUP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/followup";
/// <https://schema.org/followup>
pub const FOLLOWUP_PROPERTY_LABEL: &str = "followup";
pub struct FollowupPropertyIri;
impl PartialEq<&str> for FollowupPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOLLOWUP_PROPERTY_IRI_HTTP || *other == FOLLOWUP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FollowupPropertyIri> for &str {
	fn eq(&self, other: &FollowupPropertyIri) -> bool {
		*self == FOLLOWUP_PROPERTY_IRI_HTTP || *self == FOLLOWUP_PROPERTY_IRI_HTTPS
	}
}
pub struct FollowupPropertyIriOrLabel;
impl PartialEq<&str> for FollowupPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FollowupPropertyIri || *other == FOLLOWUP_PROPERTY_LABEL
	}
}
impl PartialEq<FollowupPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FollowupPropertyIriOrLabel) -> bool {
		*self == FollowupPropertyIri || *self == FOLLOWUP_PROPERTY_LABEL
	}
}
