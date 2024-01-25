/// <https://schema.org/HinduTemple>
pub const HINDU_TEMPLE_IRI_HTTP: &str = "http://schema.org/HinduTemple";
/// <https://schema.org/HinduTemple>
pub const HINDU_TEMPLE_IRI_HTTPS: &str = "https://schema.org/HinduTemple";
/// <https://schema.org/HinduTemple>
pub const HINDU_TEMPLE_LABEL: &str = "HinduTemple";
pub struct HinduTempleIri;
impl PartialEq<&str> for HinduTempleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HINDU_TEMPLE_IRI_HTTP || *other == HINDU_TEMPLE_IRI_HTTPS
	}
}
impl PartialEq<HinduTempleIri> for &str {
	fn eq(&self, other: &HinduTempleIri) -> bool {
		*self == HINDU_TEMPLE_IRI_HTTP || *self == HINDU_TEMPLE_IRI_HTTPS
	}
}
pub struct HinduTempleIriOrLabel;
impl PartialEq<&str> for HinduTempleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HinduTempleIri || *other == HINDU_TEMPLE_LABEL
	}
}
impl PartialEq<HinduTempleIriOrLabel> for &str {
	fn eq(&self, other: &HinduTempleIriOrLabel) -> bool {
		*self == HinduTempleIri || *self == HINDU_TEMPLE_LABEL
	}
}
