/// <https://schema.org/SpecialAnnouncement>
pub const SPECIAL_ANNOUNCEMENT_IRI_HTTP: &str = "http://schema.org/SpecialAnnouncement";
/// <https://schema.org/SpecialAnnouncement>
pub const SPECIAL_ANNOUNCEMENT_IRI_HTTPS: &str = "https://schema.org/SpecialAnnouncement";
/// <https://schema.org/SpecialAnnouncement>
pub const SPECIAL_ANNOUNCEMENT_LABEL: &str = "SpecialAnnouncement";
pub struct SpecialAnnouncementIri;
impl PartialEq<&str> for SpecialAnnouncementIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPECIAL_ANNOUNCEMENT_IRI_HTTP || *other == SPECIAL_ANNOUNCEMENT_IRI_HTTPS
	}
}
impl PartialEq<SpecialAnnouncementIri> for &str {
	fn eq(&self, other: &SpecialAnnouncementIri) -> bool {
		*self == SPECIAL_ANNOUNCEMENT_IRI_HTTP || *self == SPECIAL_ANNOUNCEMENT_IRI_HTTPS
	}
}
pub struct SpecialAnnouncementIriOrLabel;
impl PartialEq<&str> for SpecialAnnouncementIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpecialAnnouncementIri || *other == SPECIAL_ANNOUNCEMENT_LABEL
	}
}
impl PartialEq<SpecialAnnouncementIriOrLabel> for &str {
	fn eq(&self, other: &SpecialAnnouncementIriOrLabel) -> bool {
		*self == SpecialAnnouncementIri || *self == SPECIAL_ANNOUNCEMENT_LABEL
	}
}
