/// <https://schema.org/OfflineTemporarily>
pub const OFFLINE_TEMPORARILY_IRI_HTTP: &str = "http://schema.org/OfflineTemporarily";
/// <https://schema.org/OfflineTemporarily>
pub const OFFLINE_TEMPORARILY_IRI_HTTPS: &str = "https://schema.org/OfflineTemporarily";
/// <https://schema.org/OfflineTemporarily>
pub const OFFLINE_TEMPORARILY_LABEL: &str = "OfflineTemporarily";
pub struct OfflineTemporarilyIri;
impl PartialEq<&str> for OfflineTemporarilyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFLINE_TEMPORARILY_IRI_HTTP || *other == OFFLINE_TEMPORARILY_IRI_HTTPS
	}
}
impl PartialEq<OfflineTemporarilyIri> for &str {
	fn eq(&self, other: &OfflineTemporarilyIri) -> bool {
		*self == OFFLINE_TEMPORARILY_IRI_HTTP || *self == OFFLINE_TEMPORARILY_IRI_HTTPS
	}
}
pub struct OfflineTemporarilyIriOrLabel;
impl PartialEq<&str> for OfflineTemporarilyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OfflineTemporarilyIri || *other == OFFLINE_TEMPORARILY_LABEL
	}
}
impl PartialEq<OfflineTemporarilyIriOrLabel> for &str {
	fn eq(&self, other: &OfflineTemporarilyIriOrLabel) -> bool {
		*self == OfflineTemporarilyIri || *self == OFFLINE_TEMPORARILY_LABEL
	}
}
