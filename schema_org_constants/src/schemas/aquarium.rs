/// <https://schema.org/Aquarium>
pub const AQUARIUM_IRI_HTTP: &str = "http://schema.org/Aquarium";
/// <https://schema.org/Aquarium>
pub const AQUARIUM_IRI_HTTPS: &str = "https://schema.org/Aquarium";
/// <https://schema.org/Aquarium>
pub const AQUARIUM_LABEL: &str = "Aquarium";
pub struct AquariumIri;
impl PartialEq<&str> for AquariumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AQUARIUM_IRI_HTTP || *other == AQUARIUM_IRI_HTTPS
	}
}
impl PartialEq<AquariumIri> for &str {
	fn eq(&self, other: &AquariumIri) -> bool {
		*self == AQUARIUM_IRI_HTTP || *self == AQUARIUM_IRI_HTTPS
	}
}
pub struct AquariumIriOrLabel;
impl PartialEq<&str> for AquariumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AquariumIri || *other == AQUARIUM_LABEL
	}
}
impl PartialEq<AquariumIriOrLabel> for &str {
	fn eq(&self, other: &AquariumIriOrLabel) -> bool {
		*self == AquariumIri || *self == AQUARIUM_LABEL
	}
}
