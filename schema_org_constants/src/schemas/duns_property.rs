/// <https://schema.org/duns>
pub const DUNS_PROPERTY_IRI_HTTP: &str = "http://schema.org/duns";
/// <https://schema.org/duns>
pub const DUNS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/duns";
/// <https://schema.org/duns>
pub const DUNS_PROPERTY_LABEL: &str = "duns";
pub struct DunsPropertyIri;
impl PartialEq<&str> for DunsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DUNS_PROPERTY_IRI_HTTP || *other == DUNS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DunsPropertyIri> for &str {
	fn eq(&self, other: &DunsPropertyIri) -> bool {
		*self == DUNS_PROPERTY_IRI_HTTP || *self == DUNS_PROPERTY_IRI_HTTPS
	}
}
pub struct DunsPropertyIriOrLabel;
impl PartialEq<&str> for DunsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DunsPropertyIri || *other == DUNS_PROPERTY_LABEL
	}
}
impl PartialEq<DunsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DunsPropertyIriOrLabel) -> bool {
		*self == DunsPropertyIri || *self == DUNS_PROPERTY_LABEL
	}
}
