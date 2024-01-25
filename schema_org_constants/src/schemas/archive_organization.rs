/// <https://schema.org/ArchiveOrganization>
pub const ARCHIVE_ORGANIZATION_IRI_HTTP: &str = "http://schema.org/ArchiveOrganization";
/// <https://schema.org/ArchiveOrganization>
pub const ARCHIVE_ORGANIZATION_IRI_HTTPS: &str = "https://schema.org/ArchiveOrganization";
/// <https://schema.org/ArchiveOrganization>
pub const ARCHIVE_ORGANIZATION_LABEL: &str = "ArchiveOrganization";
pub struct ArchiveOrganizationIri;
impl PartialEq<&str> for ArchiveOrganizationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARCHIVE_ORGANIZATION_IRI_HTTP || *other == ARCHIVE_ORGANIZATION_IRI_HTTPS
	}
}
impl PartialEq<ArchiveOrganizationIri> for &str {
	fn eq(&self, other: &ArchiveOrganizationIri) -> bool {
		*self == ARCHIVE_ORGANIZATION_IRI_HTTP || *self == ARCHIVE_ORGANIZATION_IRI_HTTPS
	}
}
pub struct ArchiveOrganizationIriOrLabel;
impl PartialEq<&str> for ArchiveOrganizationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArchiveOrganizationIri || *other == ARCHIVE_ORGANIZATION_LABEL
	}
}
impl PartialEq<ArchiveOrganizationIriOrLabel> for &str {
	fn eq(&self, other: &ArchiveOrganizationIriOrLabel) -> bool {
		*self == ArchiveOrganizationIri || *self == ARCHIVE_ORGANIZATION_LABEL
	}
}
