/// <https://schema.org/hasDigitalDocumentPermission>
pub const HAS_DIGITAL_DOCUMENT_PERMISSION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/hasDigitalDocumentPermission";
/// <https://schema.org/hasDigitalDocumentPermission>
pub const HAS_DIGITAL_DOCUMENT_PERMISSION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/hasDigitalDocumentPermission";
/// <https://schema.org/hasDigitalDocumentPermission>
pub const HAS_DIGITAL_DOCUMENT_PERMISSION_PROPERTY_LABEL: &str = "hasDigitalDocumentPermission";
pub struct HasDigitalDocumentPermissionPropertyIri;
impl PartialEq<&str> for HasDigitalDocumentPermissionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_DIGITAL_DOCUMENT_PERMISSION_PROPERTY_IRI_HTTP
			|| *other == HAS_DIGITAL_DOCUMENT_PERMISSION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasDigitalDocumentPermissionPropertyIri> for &str {
	fn eq(&self, other: &HasDigitalDocumentPermissionPropertyIri) -> bool {
		*self == HAS_DIGITAL_DOCUMENT_PERMISSION_PROPERTY_IRI_HTTP
			|| *self == HAS_DIGITAL_DOCUMENT_PERMISSION_PROPERTY_IRI_HTTPS
	}
}
pub struct HasDigitalDocumentPermissionPropertyIriOrLabel;
impl PartialEq<&str> for HasDigitalDocumentPermissionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasDigitalDocumentPermissionPropertyIri
			|| *other == HAS_DIGITAL_DOCUMENT_PERMISSION_PROPERTY_LABEL
	}
}
impl PartialEq<HasDigitalDocumentPermissionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasDigitalDocumentPermissionPropertyIriOrLabel) -> bool {
		*self == HasDigitalDocumentPermissionPropertyIri
			|| *self == HAS_DIGITAL_DOCUMENT_PERMISSION_PROPERTY_LABEL
	}
}
