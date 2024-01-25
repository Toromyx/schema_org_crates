/// <https://schema.org/Notary>
pub const NOTARY_IRI_HTTP: &str = "http://schema.org/Notary";
/// <https://schema.org/Notary>
pub const NOTARY_IRI_HTTPS: &str = "https://schema.org/Notary";
/// <https://schema.org/Notary>
pub const NOTARY_LABEL: &str = "Notary";
pub struct NotaryIri;
impl PartialEq<&str> for NotaryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NOTARY_IRI_HTTP || *other == NOTARY_IRI_HTTPS
	}
}
impl PartialEq<NotaryIri> for &str {
	fn eq(&self, other: &NotaryIri) -> bool {
		*self == NOTARY_IRI_HTTP || *self == NOTARY_IRI_HTTPS
	}
}
pub struct NotaryIriOrLabel;
impl PartialEq<&str> for NotaryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NotaryIri || *other == NOTARY_LABEL
	}
}
impl PartialEq<NotaryIriOrLabel> for &str {
	fn eq(&self, other: &NotaryIriOrLabel) -> bool {
		*self == NotaryIri || *self == NOTARY_LABEL
	}
}
