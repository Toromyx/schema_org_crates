/// <https://schema.org/announcementLocation>
pub const ANNOUNCEMENT_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/announcementLocation";
/// <https://schema.org/announcementLocation>
pub const ANNOUNCEMENT_LOCATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/announcementLocation";
/// <https://schema.org/announcementLocation>
pub const ANNOUNCEMENT_LOCATION_PROPERTY_LABEL: &str = "announcementLocation";
pub struct AnnouncementLocationPropertyIri;
impl PartialEq<&str> for AnnouncementLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ANNOUNCEMENT_LOCATION_PROPERTY_IRI_HTTP
			|| *other == ANNOUNCEMENT_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AnnouncementLocationPropertyIri> for &str {
	fn eq(&self, other: &AnnouncementLocationPropertyIri) -> bool {
		*self == ANNOUNCEMENT_LOCATION_PROPERTY_IRI_HTTP
			|| *self == ANNOUNCEMENT_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct AnnouncementLocationPropertyIriOrLabel;
impl PartialEq<&str> for AnnouncementLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AnnouncementLocationPropertyIri || *other == ANNOUNCEMENT_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<AnnouncementLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AnnouncementLocationPropertyIriOrLabel) -> bool {
		*self == AnnouncementLocationPropertyIri || *self == ANNOUNCEMENT_LOCATION_PROPERTY_LABEL
	}
}
