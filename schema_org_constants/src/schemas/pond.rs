/// <https://schema.org/Pond>
pub const POND_IRI_HTTP: &str = "http://schema.org/Pond";
/// <https://schema.org/Pond>
pub const POND_IRI_HTTPS: &str = "https://schema.org/Pond";
/// <https://schema.org/Pond>
pub const POND_LABEL: &str = "Pond";
pub struct PondIri;
impl PartialEq<&str> for PondIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POND_IRI_HTTP || *other == POND_IRI_HTTPS
	}
}
impl PartialEq<PondIri> for &str {
	fn eq(&self, other: &PondIri) -> bool {
		*self == POND_IRI_HTTP || *self == POND_IRI_HTTPS
	}
}
pub struct PondIriOrLabel;
impl PartialEq<&str> for PondIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PondIri || *other == POND_LABEL
	}
}
impl PartialEq<PondIriOrLabel> for &str {
	fn eq(&self, other: &PondIriOrLabel) -> bool {
		*self == PondIri || *self == POND_LABEL
	}
}
