/// <https://schema.org/issn>
pub const ISSN_PROPERTY_IRI_HTTP: &str = "http://schema.org/issn";
/// <https://schema.org/issn>
pub const ISSN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/issn";
/// <https://schema.org/issn>
pub const ISSN_PROPERTY_LABEL: &str = "issn";
pub struct IssnPropertyIri;
impl PartialEq<&str> for IssnPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ISSN_PROPERTY_IRI_HTTP || *other == ISSN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IssnPropertyIri> for &str {
	fn eq(&self, other: &IssnPropertyIri) -> bool {
		*self == ISSN_PROPERTY_IRI_HTTP || *self == ISSN_PROPERTY_IRI_HTTPS
	}
}
pub struct IssnPropertyIriOrLabel;
impl PartialEq<&str> for IssnPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IssnPropertyIri || *other == ISSN_PROPERTY_LABEL
	}
}
impl PartialEq<IssnPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IssnPropertyIriOrLabel) -> bool {
		*self == IssnPropertyIri || *self == ISSN_PROPERTY_LABEL
	}
}
