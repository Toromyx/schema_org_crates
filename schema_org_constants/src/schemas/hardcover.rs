/// <https://schema.org/Hardcover>
pub const HARDCOVER_IRI_HTTP: &str = "http://schema.org/Hardcover";
/// <https://schema.org/Hardcover>
pub const HARDCOVER_IRI_HTTPS: &str = "https://schema.org/Hardcover";
/// <https://schema.org/Hardcover>
pub const HARDCOVER_LABEL: &str = "Hardcover";
pub struct HardcoverIri;
impl PartialEq<&str> for HardcoverIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HARDCOVER_IRI_HTTP || *other == HARDCOVER_IRI_HTTPS
	}
}
impl PartialEq<HardcoverIri> for &str {
	fn eq(&self, other: &HardcoverIri) -> bool {
		*self == HARDCOVER_IRI_HTTP || *self == HARDCOVER_IRI_HTTPS
	}
}
pub struct HardcoverIriOrLabel;
impl PartialEq<&str> for HardcoverIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HardcoverIri || *other == HARDCOVER_LABEL
	}
}
impl PartialEq<HardcoverIriOrLabel> for &str {
	fn eq(&self, other: &HardcoverIriOrLabel) -> bool {
		*self == HardcoverIri || *self == HARDCOVER_LABEL
	}
}
