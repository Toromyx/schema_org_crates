/// <https://schema.org/headline>
pub const HEADLINE_PROPERTY_IRI_HTTP: &str = "http://schema.org/headline";
/// <https://schema.org/headline>
pub const HEADLINE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/headline";
/// <https://schema.org/headline>
pub const HEADLINE_PROPERTY_LABEL: &str = "headline";
pub struct HeadlinePropertyIri;
impl PartialEq<&str> for HeadlinePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEADLINE_PROPERTY_IRI_HTTP || *other == HEADLINE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HeadlinePropertyIri> for &str {
	fn eq(&self, other: &HeadlinePropertyIri) -> bool {
		*self == HEADLINE_PROPERTY_IRI_HTTP || *self == HEADLINE_PROPERTY_IRI_HTTPS
	}
}
pub struct HeadlinePropertyIriOrLabel;
impl PartialEq<&str> for HeadlinePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HeadlinePropertyIri || *other == HEADLINE_PROPERTY_LABEL
	}
}
impl PartialEq<HeadlinePropertyIriOrLabel> for &str {
	fn eq(&self, other: &HeadlinePropertyIriOrLabel) -> bool {
		*self == HeadlinePropertyIri || *self == HEADLINE_PROPERTY_LABEL
	}
}
