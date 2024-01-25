/// <https://schema.org/DigitalDocumentPermissionType>
pub const DIGITAL_DOCUMENT_PERMISSION_TYPE_IRI_HTTP: &str =
	"http://schema.org/DigitalDocumentPermissionType";
/// <https://schema.org/DigitalDocumentPermissionType>
pub const DIGITAL_DOCUMENT_PERMISSION_TYPE_IRI_HTTPS: &str =
	"https://schema.org/DigitalDocumentPermissionType";
/// <https://schema.org/DigitalDocumentPermissionType>
pub const DIGITAL_DOCUMENT_PERMISSION_TYPE_LABEL: &str = "DigitalDocumentPermissionType";
pub struct DigitalDocumentPermissionTypeIri;
impl PartialEq<&str> for DigitalDocumentPermissionTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIGITAL_DOCUMENT_PERMISSION_TYPE_IRI_HTTP
			|| *other == DIGITAL_DOCUMENT_PERMISSION_TYPE_IRI_HTTPS
	}
}
impl PartialEq<DigitalDocumentPermissionTypeIri> for &str {
	fn eq(&self, other: &DigitalDocumentPermissionTypeIri) -> bool {
		*self == DIGITAL_DOCUMENT_PERMISSION_TYPE_IRI_HTTP
			|| *self == DIGITAL_DOCUMENT_PERMISSION_TYPE_IRI_HTTPS
	}
}
pub struct DigitalDocumentPermissionTypeIriOrLabel;
impl PartialEq<&str> for DigitalDocumentPermissionTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DigitalDocumentPermissionTypeIri
			|| *other == DIGITAL_DOCUMENT_PERMISSION_TYPE_LABEL
	}
}
impl PartialEq<DigitalDocumentPermissionTypeIriOrLabel> for &str {
	fn eq(&self, other: &DigitalDocumentPermissionTypeIriOrLabel) -> bool {
		*self == DigitalDocumentPermissionTypeIri || *self == DIGITAL_DOCUMENT_PERMISSION_TYPE_LABEL
	}
}
