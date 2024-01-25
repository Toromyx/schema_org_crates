/// <https://schema.org/Mountain>
pub const MOUNTAIN_IRI_HTTP: &str = "http://schema.org/Mountain";
/// <https://schema.org/Mountain>
pub const MOUNTAIN_IRI_HTTPS: &str = "https://schema.org/Mountain";
/// <https://schema.org/Mountain>
pub const MOUNTAIN_LABEL: &str = "Mountain";
pub struct MountainIri;
impl PartialEq<&str> for MountainIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOUNTAIN_IRI_HTTP || *other == MOUNTAIN_IRI_HTTPS
	}
}
impl PartialEq<MountainIri> for &str {
	fn eq(&self, other: &MountainIri) -> bool {
		*self == MOUNTAIN_IRI_HTTP || *self == MOUNTAIN_IRI_HTTPS
	}
}
pub struct MountainIriOrLabel;
impl PartialEq<&str> for MountainIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MountainIri || *other == MOUNTAIN_LABEL
	}
}
impl PartialEq<MountainIriOrLabel> for &str {
	fn eq(&self, other: &MountainIriOrLabel) -> bool {
		*self == MountainIri || *self == MOUNTAIN_LABEL
	}
}
