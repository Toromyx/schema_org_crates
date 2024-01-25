/// <https://schema.org/Oncologic>
pub const ONCOLOGIC_IRI_HTTP: &str = "http://schema.org/Oncologic";
/// <https://schema.org/Oncologic>
pub const ONCOLOGIC_IRI_HTTPS: &str = "https://schema.org/Oncologic";
/// <https://schema.org/Oncologic>
pub const ONCOLOGIC_LABEL: &str = "Oncologic";
pub struct OncologicIri;
impl PartialEq<&str> for OncologicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ONCOLOGIC_IRI_HTTP || *other == ONCOLOGIC_IRI_HTTPS
	}
}
impl PartialEq<OncologicIri> for &str {
	fn eq(&self, other: &OncologicIri) -> bool {
		*self == ONCOLOGIC_IRI_HTTP || *self == ONCOLOGIC_IRI_HTTPS
	}
}
pub struct OncologicIriOrLabel;
impl PartialEq<&str> for OncologicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OncologicIri || *other == ONCOLOGIC_LABEL
	}
}
impl PartialEq<OncologicIriOrLabel> for &str {
	fn eq(&self, other: &OncologicIriOrLabel) -> bool {
		*self == OncologicIri || *self == ONCOLOGIC_LABEL
	}
}
