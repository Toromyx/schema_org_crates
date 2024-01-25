/// <https://schema.org/OfflinePermanently>
pub const OFFLINE_PERMANENTLY_IRI_HTTP: &str = "http://schema.org/OfflinePermanently";
/// <https://schema.org/OfflinePermanently>
pub const OFFLINE_PERMANENTLY_IRI_HTTPS: &str = "https://schema.org/OfflinePermanently";
/// <https://schema.org/OfflinePermanently>
pub const OFFLINE_PERMANENTLY_LABEL: &str = "OfflinePermanently";
pub struct OfflinePermanentlyIri;
impl PartialEq<&str> for OfflinePermanentlyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFLINE_PERMANENTLY_IRI_HTTP || *other == OFFLINE_PERMANENTLY_IRI_HTTPS
	}
}
impl PartialEq<OfflinePermanentlyIri> for &str {
	fn eq(&self, other: &OfflinePermanentlyIri) -> bool {
		*self == OFFLINE_PERMANENTLY_IRI_HTTP || *self == OFFLINE_PERMANENTLY_IRI_HTTPS
	}
}
pub struct OfflinePermanentlyIriOrLabel;
impl PartialEq<&str> for OfflinePermanentlyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OfflinePermanentlyIri || *other == OFFLINE_PERMANENTLY_LABEL
	}
}
impl PartialEq<OfflinePermanentlyIriOrLabel> for &str {
	fn eq(&self, other: &OfflinePermanentlyIriOrLabel) -> bool {
		*self == OfflinePermanentlyIri || *self == OFFLINE_PERMANENTLY_LABEL
	}
}
