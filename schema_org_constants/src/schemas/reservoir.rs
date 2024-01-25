/// <https://schema.org/Reservoir>
pub const RESERVOIR_IRI_HTTP: &str = "http://schema.org/Reservoir";
/// <https://schema.org/Reservoir>
pub const RESERVOIR_IRI_HTTPS: &str = "https://schema.org/Reservoir";
/// <https://schema.org/Reservoir>
pub const RESERVOIR_LABEL: &str = "Reservoir";
pub struct ReservoirIri;
impl PartialEq<&str> for ReservoirIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESERVOIR_IRI_HTTP || *other == RESERVOIR_IRI_HTTPS
	}
}
impl PartialEq<ReservoirIri> for &str {
	fn eq(&self, other: &ReservoirIri) -> bool {
		*self == RESERVOIR_IRI_HTTP || *self == RESERVOIR_IRI_HTTPS
	}
}
pub struct ReservoirIriOrLabel;
impl PartialEq<&str> for ReservoirIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReservoirIri || *other == RESERVOIR_LABEL
	}
}
impl PartialEq<ReservoirIriOrLabel> for &str {
	fn eq(&self, other: &ReservoirIriOrLabel) -> bool {
		*self == ReservoirIri || *self == RESERVOIR_LABEL
	}
}
