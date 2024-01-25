/// <https://schema.org/storageRequirements>
pub const STORAGE_REQUIREMENTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/storageRequirements";
/// <https://schema.org/storageRequirements>
pub const STORAGE_REQUIREMENTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/storageRequirements";
/// <https://schema.org/storageRequirements>
pub const STORAGE_REQUIREMENTS_PROPERTY_LABEL: &str = "storageRequirements";
pub struct StorageRequirementsPropertyIri;
impl PartialEq<&str> for StorageRequirementsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STORAGE_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *other == STORAGE_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StorageRequirementsPropertyIri> for &str {
	fn eq(&self, other: &StorageRequirementsPropertyIri) -> bool {
		*self == STORAGE_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *self == STORAGE_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct StorageRequirementsPropertyIriOrLabel;
impl PartialEq<&str> for StorageRequirementsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StorageRequirementsPropertyIri || *other == STORAGE_REQUIREMENTS_PROPERTY_LABEL
	}
}
impl PartialEq<StorageRequirementsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StorageRequirementsPropertyIriOrLabel) -> bool {
		*self == StorageRequirementsPropertyIri || *self == STORAGE_REQUIREMENTS_PROPERTY_LABEL
	}
}
