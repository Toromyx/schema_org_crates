/// <https://schema.org/AerobicActivity>
pub const AEROBIC_ACTIVITY_IRI_HTTP: &str = "http://schema.org/AerobicActivity";
/// <https://schema.org/AerobicActivity>
pub const AEROBIC_ACTIVITY_IRI_HTTPS: &str = "https://schema.org/AerobicActivity";
/// <https://schema.org/AerobicActivity>
pub const AEROBIC_ACTIVITY_LABEL: &str = "AerobicActivity";
pub struct AerobicActivityIri;
impl PartialEq<&str> for AerobicActivityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AEROBIC_ACTIVITY_IRI_HTTP || *other == AEROBIC_ACTIVITY_IRI_HTTPS
	}
}
impl PartialEq<AerobicActivityIri> for &str {
	fn eq(&self, other: &AerobicActivityIri) -> bool {
		*self == AEROBIC_ACTIVITY_IRI_HTTP || *self == AEROBIC_ACTIVITY_IRI_HTTPS
	}
}
pub struct AerobicActivityIriOrLabel;
impl PartialEq<&str> for AerobicActivityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AerobicActivityIri || *other == AEROBIC_ACTIVITY_LABEL
	}
}
impl PartialEq<AerobicActivityIriOrLabel> for &str {
	fn eq(&self, other: &AerobicActivityIriOrLabel) -> bool {
		*self == AerobicActivityIri || *self == AEROBIC_ACTIVITY_LABEL
	}
}
