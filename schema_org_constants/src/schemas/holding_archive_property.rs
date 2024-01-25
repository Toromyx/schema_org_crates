/// <https://schema.org/holdingArchive>
pub const HOLDING_ARCHIVE_PROPERTY_IRI_HTTP: &str = "http://schema.org/holdingArchive";
/// <https://schema.org/holdingArchive>
pub const HOLDING_ARCHIVE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/holdingArchive";
/// <https://schema.org/holdingArchive>
pub const HOLDING_ARCHIVE_PROPERTY_LABEL: &str = "holdingArchive";
pub struct HoldingArchivePropertyIri;
impl PartialEq<&str> for HoldingArchivePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOLDING_ARCHIVE_PROPERTY_IRI_HTTP || *other == HOLDING_ARCHIVE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HoldingArchivePropertyIri> for &str {
	fn eq(&self, other: &HoldingArchivePropertyIri) -> bool {
		*self == HOLDING_ARCHIVE_PROPERTY_IRI_HTTP || *self == HOLDING_ARCHIVE_PROPERTY_IRI_HTTPS
	}
}
pub struct HoldingArchivePropertyIriOrLabel;
impl PartialEq<&str> for HoldingArchivePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HoldingArchivePropertyIri || *other == HOLDING_ARCHIVE_PROPERTY_LABEL
	}
}
impl PartialEq<HoldingArchivePropertyIriOrLabel> for &str {
	fn eq(&self, other: &HoldingArchivePropertyIriOrLabel) -> bool {
		*self == HoldingArchivePropertyIri || *self == HOLDING_ARCHIVE_PROPERTY_LABEL
	}
}
