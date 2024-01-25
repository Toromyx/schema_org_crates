/// <https://schema.org/ArchiveComponent>
pub const ARCHIVE_COMPONENT_IRI_HTTP: &str = "http://schema.org/ArchiveComponent";
/// <https://schema.org/ArchiveComponent>
pub const ARCHIVE_COMPONENT_IRI_HTTPS: &str = "https://schema.org/ArchiveComponent";
/// <https://schema.org/ArchiveComponent>
pub const ARCHIVE_COMPONENT_LABEL: &str = "ArchiveComponent";
pub struct ArchiveComponentIri;
impl PartialEq<&str> for ArchiveComponentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARCHIVE_COMPONENT_IRI_HTTP || *other == ARCHIVE_COMPONENT_IRI_HTTPS
	}
}
impl PartialEq<ArchiveComponentIri> for &str {
	fn eq(&self, other: &ArchiveComponentIri) -> bool {
		*self == ARCHIVE_COMPONENT_IRI_HTTP || *self == ARCHIVE_COMPONENT_IRI_HTTPS
	}
}
pub struct ArchiveComponentIriOrLabel;
impl PartialEq<&str> for ArchiveComponentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArchiveComponentIri || *other == ARCHIVE_COMPONENT_LABEL
	}
}
impl PartialEq<ArchiveComponentIriOrLabel> for &str {
	fn eq(&self, other: &ArchiveComponentIriOrLabel) -> bool {
		*self == ArchiveComponentIri || *self == ARCHIVE_COMPONENT_LABEL
	}
}
