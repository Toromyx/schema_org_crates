/// <https://schema.org/BuddhistTemple>
pub const BUDDHIST_TEMPLE_IRI_HTTP: &str = "http://schema.org/BuddhistTemple";
/// <https://schema.org/BuddhistTemple>
pub const BUDDHIST_TEMPLE_IRI_HTTPS: &str = "https://schema.org/BuddhistTemple";
/// <https://schema.org/BuddhistTemple>
pub const BUDDHIST_TEMPLE_LABEL: &str = "BuddhistTemple";
pub struct BuddhistTempleIri;
impl PartialEq<&str> for BuddhistTempleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUDDHIST_TEMPLE_IRI_HTTP || *other == BUDDHIST_TEMPLE_IRI_HTTPS
	}
}
impl PartialEq<BuddhistTempleIri> for &str {
	fn eq(&self, other: &BuddhistTempleIri) -> bool {
		*self == BUDDHIST_TEMPLE_IRI_HTTP || *self == BUDDHIST_TEMPLE_IRI_HTTPS
	}
}
pub struct BuddhistTempleIriOrLabel;
impl PartialEq<&str> for BuddhistTempleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BuddhistTempleIri || *other == BUDDHIST_TEMPLE_LABEL
	}
}
impl PartialEq<BuddhistTempleIriOrLabel> for &str {
	fn eq(&self, other: &BuddhistTempleIriOrLabel) -> bool {
		*self == BuddhistTempleIri || *self == BUDDHIST_TEMPLE_LABEL
	}
}
