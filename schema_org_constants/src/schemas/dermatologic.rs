/// <https://schema.org/Dermatologic>
#[deprecated = "This schema is superseded by <https://schema.org/Dermatology>."]
pub const DERMATOLOGIC_IRI_HTTP: &str = "http://schema.org/Dermatologic";
/// <https://schema.org/Dermatologic>
#[deprecated = "This schema is superseded by <https://schema.org/Dermatology>."]
pub const DERMATOLOGIC_IRI_HTTPS: &str = "https://schema.org/Dermatologic";
/// <https://schema.org/Dermatologic>
#[deprecated = "This schema is superseded by <https://schema.org/Dermatology>."]
pub const DERMATOLOGIC_LABEL: &str = "Dermatologic";
pub struct DermatologicIri;
impl PartialEq<&str> for DermatologicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DERMATOLOGIC_IRI_HTTP || *other == DERMATOLOGIC_IRI_HTTPS
	}
}
impl PartialEq<DermatologicIri> for &str {
	fn eq(&self, other: &DermatologicIri) -> bool {
		*self == DERMATOLOGIC_IRI_HTTP || *self == DERMATOLOGIC_IRI_HTTPS
	}
}
pub struct DermatologicIriOrLabel;
impl PartialEq<&str> for DermatologicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DermatologicIri || *other == DERMATOLOGIC_LABEL
	}
}
impl PartialEq<DermatologicIriOrLabel> for &str {
	fn eq(&self, other: &DermatologicIriOrLabel) -> bool {
		*self == DermatologicIri || *self == DERMATOLOGIC_LABEL
	}
}
