/// <https://schema.org/archiveHeld>
pub const ARCHIVE_HELD_PROPERTY_IRI_HTTP: &str = "http://schema.org/archiveHeld";
/// <https://schema.org/archiveHeld>
pub const ARCHIVE_HELD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/archiveHeld";
/// <https://schema.org/archiveHeld>
pub const ARCHIVE_HELD_PROPERTY_LABEL: &str = "archiveHeld";
pub struct ArchiveHeldPropertyIri;
impl PartialEq<&str> for ArchiveHeldPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARCHIVE_HELD_PROPERTY_IRI_HTTP || *other == ARCHIVE_HELD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArchiveHeldPropertyIri> for &str {
	fn eq(&self, other: &ArchiveHeldPropertyIri) -> bool {
		*self == ARCHIVE_HELD_PROPERTY_IRI_HTTP || *self == ARCHIVE_HELD_PROPERTY_IRI_HTTPS
	}
}
pub struct ArchiveHeldPropertyIriOrLabel;
impl PartialEq<&str> for ArchiveHeldPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArchiveHeldPropertyIri || *other == ARCHIVE_HELD_PROPERTY_LABEL
	}
}
impl PartialEq<ArchiveHeldPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArchiveHeldPropertyIriOrLabel) -> bool {
		*self == ArchiveHeldPropertyIri || *self == ARCHIVE_HELD_PROPERTY_LABEL
	}
}
