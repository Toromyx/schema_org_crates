/// <https://schema.org/vendor>
#[deprecated = "This schema is superseded by <https://schema.org/seller>."]
pub const VENDOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/vendor";
/// <https://schema.org/vendor>
#[deprecated = "This schema is superseded by <https://schema.org/seller>."]
pub const VENDOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/vendor";
/// <https://schema.org/vendor>
#[deprecated = "This schema is superseded by <https://schema.org/seller>."]
pub const VENDOR_PROPERTY_LABEL: &str = "vendor";
pub struct VendorPropertyIri;
impl PartialEq<&str> for VendorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VENDOR_PROPERTY_IRI_HTTP || *other == VENDOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VendorPropertyIri> for &str {
	fn eq(&self, other: &VendorPropertyIri) -> bool {
		*self == VENDOR_PROPERTY_IRI_HTTP || *self == VENDOR_PROPERTY_IRI_HTTPS
	}
}
pub struct VendorPropertyIriOrLabel;
impl PartialEq<&str> for VendorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VendorPropertyIri || *other == VENDOR_PROPERTY_LABEL
	}
}
impl PartialEq<VendorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VendorPropertyIriOrLabel) -> bool {
		*self == VendorPropertyIri || *self == VENDOR_PROPERTY_LABEL
	}
}
