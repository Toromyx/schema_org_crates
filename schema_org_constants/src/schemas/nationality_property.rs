/// <https://schema.org/nationality>
pub const NATIONALITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/nationality";
/// <https://schema.org/nationality>
pub const NATIONALITY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/nationality";
/// <https://schema.org/nationality>
pub const NATIONALITY_PROPERTY_LABEL: &str = "nationality";
pub struct NationalityPropertyIri;
impl PartialEq<&str> for NationalityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NATIONALITY_PROPERTY_IRI_HTTP || *other == NATIONALITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NationalityPropertyIri> for &str {
	fn eq(&self, other: &NationalityPropertyIri) -> bool {
		*self == NATIONALITY_PROPERTY_IRI_HTTP || *self == NATIONALITY_PROPERTY_IRI_HTTPS
	}
}
pub struct NationalityPropertyIriOrLabel;
impl PartialEq<&str> for NationalityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NationalityPropertyIri || *other == NATIONALITY_PROPERTY_LABEL
	}
}
impl PartialEq<NationalityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NationalityPropertyIriOrLabel) -> bool {
		*self == NationalityPropertyIri || *self == NATIONALITY_PROPERTY_LABEL
	}
}
