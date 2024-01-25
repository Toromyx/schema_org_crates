/// <https://schema.org/Dermatology>
pub const DERMATOLOGY_IRI_HTTP: &str = "http://schema.org/Dermatology";
/// <https://schema.org/Dermatology>
pub const DERMATOLOGY_IRI_HTTPS: &str = "https://schema.org/Dermatology";
/// <https://schema.org/Dermatology>
pub const DERMATOLOGY_LABEL: &str = "Dermatology";
pub struct DermatologyIri;
impl PartialEq<&str> for DermatologyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DERMATOLOGY_IRI_HTTP || *other == DERMATOLOGY_IRI_HTTPS
	}
}
impl PartialEq<DermatologyIri> for &str {
	fn eq(&self, other: &DermatologyIri) -> bool {
		*self == DERMATOLOGY_IRI_HTTP || *self == DERMATOLOGY_IRI_HTTPS
	}
}
pub struct DermatologyIriOrLabel;
impl PartialEq<&str> for DermatologyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DermatologyIri || *other == DERMATOLOGY_LABEL
	}
}
impl PartialEq<DermatologyIriOrLabel> for &str {
	fn eq(&self, other: &DermatologyIriOrLabel) -> bool {
		*self == DermatologyIri || *self == DERMATOLOGY_LABEL
	}
}
