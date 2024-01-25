/// <https://schema.org/VinylFormat>
pub const VINYL_FORMAT_IRI_HTTP: &str = "http://schema.org/VinylFormat";
/// <https://schema.org/VinylFormat>
pub const VINYL_FORMAT_IRI_HTTPS: &str = "https://schema.org/VinylFormat";
/// <https://schema.org/VinylFormat>
pub const VINYL_FORMAT_LABEL: &str = "VinylFormat";
pub struct VinylFormatIri;
impl PartialEq<&str> for VinylFormatIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VINYL_FORMAT_IRI_HTTP || *other == VINYL_FORMAT_IRI_HTTPS
	}
}
impl PartialEq<VinylFormatIri> for &str {
	fn eq(&self, other: &VinylFormatIri) -> bool {
		*self == VINYL_FORMAT_IRI_HTTP || *self == VINYL_FORMAT_IRI_HTTPS
	}
}
pub struct VinylFormatIriOrLabel;
impl PartialEq<&str> for VinylFormatIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VinylFormatIri || *other == VINYL_FORMAT_LABEL
	}
}
impl PartialEq<VinylFormatIriOrLabel> for &str {
	fn eq(&self, other: &VinylFormatIriOrLabel) -> bool {
		*self == VinylFormatIri || *self == VINYL_FORMAT_LABEL
	}
}
