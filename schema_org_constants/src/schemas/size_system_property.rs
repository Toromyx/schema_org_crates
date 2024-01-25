/// <https://schema.org/sizeSystem>
pub const SIZE_SYSTEM_PROPERTY_IRI_HTTP: &str = "http://schema.org/sizeSystem";
/// <https://schema.org/sizeSystem>
pub const SIZE_SYSTEM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sizeSystem";
/// <https://schema.org/sizeSystem>
pub const SIZE_SYSTEM_PROPERTY_LABEL: &str = "sizeSystem";
pub struct SizeSystemPropertyIri;
impl PartialEq<&str> for SizeSystemPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIZE_SYSTEM_PROPERTY_IRI_HTTP || *other == SIZE_SYSTEM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SizeSystemPropertyIri> for &str {
	fn eq(&self, other: &SizeSystemPropertyIri) -> bool {
		*self == SIZE_SYSTEM_PROPERTY_IRI_HTTP || *self == SIZE_SYSTEM_PROPERTY_IRI_HTTPS
	}
}
pub struct SizeSystemPropertyIriOrLabel;
impl PartialEq<&str> for SizeSystemPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SizeSystemPropertyIri || *other == SIZE_SYSTEM_PROPERTY_LABEL
	}
}
impl PartialEq<SizeSystemPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SizeSystemPropertyIriOrLabel) -> bool {
		*self == SizeSystemPropertyIri || *self == SIZE_SYSTEM_PROPERTY_LABEL
	}
}
