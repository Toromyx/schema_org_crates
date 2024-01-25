/// <https://schema.org/DigitalDocumentPermission>
pub const DIGITAL_DOCUMENT_PERMISSION_IRI_HTTP: &str =
	"http://schema.org/DigitalDocumentPermission";
/// <https://schema.org/DigitalDocumentPermission>
pub const DIGITAL_DOCUMENT_PERMISSION_IRI_HTTPS: &str =
	"https://schema.org/DigitalDocumentPermission";
/// <https://schema.org/DigitalDocumentPermission>
pub const DIGITAL_DOCUMENT_PERMISSION_LABEL: &str = "DigitalDocumentPermission";
pub struct DigitalDocumentPermissionIri;
impl PartialEq<&str> for DigitalDocumentPermissionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIGITAL_DOCUMENT_PERMISSION_IRI_HTTP
			|| *other == DIGITAL_DOCUMENT_PERMISSION_IRI_HTTPS
	}
}
impl PartialEq<DigitalDocumentPermissionIri> for &str {
	fn eq(&self, other: &DigitalDocumentPermissionIri) -> bool {
		*self == DIGITAL_DOCUMENT_PERMISSION_IRI_HTTP
			|| *self == DIGITAL_DOCUMENT_PERMISSION_IRI_HTTPS
	}
}
pub struct DigitalDocumentPermissionIriOrLabel;
impl PartialEq<&str> for DigitalDocumentPermissionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DigitalDocumentPermissionIri || *other == DIGITAL_DOCUMENT_PERMISSION_LABEL
	}
}
impl PartialEq<DigitalDocumentPermissionIriOrLabel> for &str {
	fn eq(&self, other: &DigitalDocumentPermissionIriOrLabel) -> bool {
		*self == DigitalDocumentPermissionIri || *self == DIGITAL_DOCUMENT_PERMISSION_LABEL
	}
}
