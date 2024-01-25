/// <https://schema.org/SRP>
pub const SRP_IRI_HTTP: &str = "http://schema.org/SRP";
/// <https://schema.org/SRP>
pub const SRP_IRI_HTTPS: &str = "https://schema.org/SRP";
/// <https://schema.org/SRP>
pub const SRP_LABEL: &str = "SRP";
pub struct SrpIri;
impl PartialEq<&str> for SrpIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SRP_IRI_HTTP || *other == SRP_IRI_HTTPS
	}
}
impl PartialEq<SrpIri> for &str {
	fn eq(&self, other: &SrpIri) -> bool {
		*self == SRP_IRI_HTTP || *self == SRP_IRI_HTTPS
	}
}
pub struct SrpIriOrLabel;
impl PartialEq<&str> for SrpIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SrpIri || *other == SRP_LABEL
	}
}
impl PartialEq<SrpIriOrLabel> for &str {
	fn eq(&self, other: &SrpIriOrLabel) -> bool {
		*self == SrpIri || *self == SRP_LABEL
	}
}
