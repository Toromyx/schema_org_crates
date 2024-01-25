/// <https://schema.org/SizeSystemImperial>
pub const SIZE_SYSTEM_IMPERIAL_IRI_HTTP: &str = "http://schema.org/SizeSystemImperial";
/// <https://schema.org/SizeSystemImperial>
pub const SIZE_SYSTEM_IMPERIAL_IRI_HTTPS: &str = "https://schema.org/SizeSystemImperial";
/// <https://schema.org/SizeSystemImperial>
pub const SIZE_SYSTEM_IMPERIAL_LABEL: &str = "SizeSystemImperial";
pub struct SizeSystemImperialIri;
impl PartialEq<&str> for SizeSystemImperialIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIZE_SYSTEM_IMPERIAL_IRI_HTTP || *other == SIZE_SYSTEM_IMPERIAL_IRI_HTTPS
	}
}
impl PartialEq<SizeSystemImperialIri> for &str {
	fn eq(&self, other: &SizeSystemImperialIri) -> bool {
		*self == SIZE_SYSTEM_IMPERIAL_IRI_HTTP || *self == SIZE_SYSTEM_IMPERIAL_IRI_HTTPS
	}
}
pub struct SizeSystemImperialIriOrLabel;
impl PartialEq<&str> for SizeSystemImperialIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SizeSystemImperialIri || *other == SIZE_SYSTEM_IMPERIAL_LABEL
	}
}
impl PartialEq<SizeSystemImperialIriOrLabel> for &str {
	fn eq(&self, other: &SizeSystemImperialIriOrLabel) -> bool {
		*self == SizeSystemImperialIri || *self == SIZE_SYSTEM_IMPERIAL_LABEL
	}
}
