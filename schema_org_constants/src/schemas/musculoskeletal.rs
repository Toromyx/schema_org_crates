/// <https://schema.org/Musculoskeletal>
pub const MUSCULOSKELETAL_IRI_HTTP: &str = "http://schema.org/Musculoskeletal";
/// <https://schema.org/Musculoskeletal>
pub const MUSCULOSKELETAL_IRI_HTTPS: &str = "https://schema.org/Musculoskeletal";
/// <https://schema.org/Musculoskeletal>
pub const MUSCULOSKELETAL_LABEL: &str = "Musculoskeletal";
pub struct MusculoskeletalIri;
impl PartialEq<&str> for MusculoskeletalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSCULOSKELETAL_IRI_HTTP || *other == MUSCULOSKELETAL_IRI_HTTPS
	}
}
impl PartialEq<MusculoskeletalIri> for &str {
	fn eq(&self, other: &MusculoskeletalIri) -> bool {
		*self == MUSCULOSKELETAL_IRI_HTTP || *self == MUSCULOSKELETAL_IRI_HTTPS
	}
}
pub struct MusculoskeletalIriOrLabel;
impl PartialEq<&str> for MusculoskeletalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusculoskeletalIri || *other == MUSCULOSKELETAL_LABEL
	}
}
impl PartialEq<MusculoskeletalIriOrLabel> for &str {
	fn eq(&self, other: &MusculoskeletalIriOrLabel) -> bool {
		*self == MusculoskeletalIri || *self == MUSCULOSKELETAL_LABEL
	}
}
