/// <https://schema.org/Vein>
pub const VEIN_IRI_HTTP: &str = "http://schema.org/Vein";
/// <https://schema.org/Vein>
pub const VEIN_IRI_HTTPS: &str = "https://schema.org/Vein";
/// <https://schema.org/Vein>
pub const VEIN_LABEL: &str = "Vein";
pub struct VeinIri;
impl PartialEq<&str> for VeinIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VEIN_IRI_HTTP || *other == VEIN_IRI_HTTPS
	}
}
impl PartialEq<VeinIri> for &str {
	fn eq(&self, other: &VeinIri) -> bool {
		*self == VEIN_IRI_HTTP || *self == VEIN_IRI_HTTPS
	}
}
pub struct VeinIriOrLabel;
impl PartialEq<&str> for VeinIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VeinIri || *other == VEIN_LABEL
	}
}
impl PartialEq<VeinIriOrLabel> for &str {
	fn eq(&self, other: &VeinIriOrLabel) -> bool {
		*self == VeinIri || *self == VEIN_LABEL
	}
}
