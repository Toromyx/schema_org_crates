/// <https://schema.org/Urologic>
pub const UROLOGIC_IRI_HTTP: &str = "http://schema.org/Urologic";
/// <https://schema.org/Urologic>
pub const UROLOGIC_IRI_HTTPS: &str = "https://schema.org/Urologic";
/// <https://schema.org/Urologic>
pub const UROLOGIC_LABEL: &str = "Urologic";
pub struct UrologicIri;
impl PartialEq<&str> for UrologicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UROLOGIC_IRI_HTTP || *other == UROLOGIC_IRI_HTTPS
	}
}
impl PartialEq<UrologicIri> for &str {
	fn eq(&self, other: &UrologicIri) -> bool {
		*self == UROLOGIC_IRI_HTTP || *self == UROLOGIC_IRI_HTTPS
	}
}
pub struct UrologicIriOrLabel;
impl PartialEq<&str> for UrologicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UrologicIri || *other == UROLOGIC_LABEL
	}
}
impl PartialEq<UrologicIriOrLabel> for &str {
	fn eq(&self, other: &UrologicIriOrLabel) -> bool {
		*self == UrologicIri || *self == UROLOGIC_LABEL
	}
}
