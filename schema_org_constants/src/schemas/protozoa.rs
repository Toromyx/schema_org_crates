/// <https://schema.org/Protozoa>
pub const PROTOZOA_IRI_HTTP: &str = "http://schema.org/Protozoa";
/// <https://schema.org/Protozoa>
pub const PROTOZOA_IRI_HTTPS: &str = "https://schema.org/Protozoa";
/// <https://schema.org/Protozoa>
pub const PROTOZOA_LABEL: &str = "Protozoa";
pub struct ProtozoaIri;
impl PartialEq<&str> for ProtozoaIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROTOZOA_IRI_HTTP || *other == PROTOZOA_IRI_HTTPS
	}
}
impl PartialEq<ProtozoaIri> for &str {
	fn eq(&self, other: &ProtozoaIri) -> bool {
		*self == PROTOZOA_IRI_HTTP || *self == PROTOZOA_IRI_HTTPS
	}
}
pub struct ProtozoaIriOrLabel;
impl PartialEq<&str> for ProtozoaIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProtozoaIri || *other == PROTOZOA_LABEL
	}
}
impl PartialEq<ProtozoaIriOrLabel> for &str {
	fn eq(&self, other: &ProtozoaIriOrLabel) -> bool {
		*self == ProtozoaIri || *self == PROTOZOA_LABEL
	}
}
