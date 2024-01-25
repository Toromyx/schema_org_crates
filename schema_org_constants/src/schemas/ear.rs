/// <https://schema.org/Ear>
pub const EAR_IRI_HTTP: &str = "http://schema.org/Ear";
/// <https://schema.org/Ear>
pub const EAR_IRI_HTTPS: &str = "https://schema.org/Ear";
/// <https://schema.org/Ear>
pub const EAR_LABEL: &str = "Ear";
pub struct EarIri;
impl PartialEq<&str> for EarIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EAR_IRI_HTTP || *other == EAR_IRI_HTTPS
	}
}
impl PartialEq<EarIri> for &str {
	fn eq(&self, other: &EarIri) -> bool {
		*self == EAR_IRI_HTTP || *self == EAR_IRI_HTTPS
	}
}
pub struct EarIriOrLabel;
impl PartialEq<&str> for EarIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EarIri || *other == EAR_LABEL
	}
}
impl PartialEq<EarIriOrLabel> for &str {
	fn eq(&self, other: &EarIriOrLabel) -> bool {
		*self == EarIri || *self == EAR_LABEL
	}
}
