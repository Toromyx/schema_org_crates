/// <https://schema.org/penciler>
pub const PENCILER_PROPERTY_IRI_HTTP: &str = "http://schema.org/penciler";
/// <https://schema.org/penciler>
pub const PENCILER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/penciler";
/// <https://schema.org/penciler>
pub const PENCILER_PROPERTY_LABEL: &str = "penciler";
pub struct PencilerPropertyIri;
impl PartialEq<&str> for PencilerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PENCILER_PROPERTY_IRI_HTTP || *other == PENCILER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PencilerPropertyIri> for &str {
	fn eq(&self, other: &PencilerPropertyIri) -> bool {
		*self == PENCILER_PROPERTY_IRI_HTTP || *self == PENCILER_PROPERTY_IRI_HTTPS
	}
}
pub struct PencilerPropertyIriOrLabel;
impl PartialEq<&str> for PencilerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PencilerPropertyIri || *other == PENCILER_PROPERTY_LABEL
	}
}
impl PartialEq<PencilerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PencilerPropertyIriOrLabel) -> bool {
		*self == PencilerPropertyIri || *self == PENCILER_PROPERTY_LABEL
	}
}
