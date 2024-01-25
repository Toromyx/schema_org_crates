/// <https://schema.org/replacee>
pub const REPLACEE_PROPERTY_IRI_HTTP: &str = "http://schema.org/replacee";
/// <https://schema.org/replacee>
pub const REPLACEE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/replacee";
/// <https://schema.org/replacee>
pub const REPLACEE_PROPERTY_LABEL: &str = "replacee";
pub struct ReplaceePropertyIri;
impl PartialEq<&str> for ReplaceePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REPLACEE_PROPERTY_IRI_HTTP || *other == REPLACEE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReplaceePropertyIri> for &str {
	fn eq(&self, other: &ReplaceePropertyIri) -> bool {
		*self == REPLACEE_PROPERTY_IRI_HTTP || *self == REPLACEE_PROPERTY_IRI_HTTPS
	}
}
pub struct ReplaceePropertyIriOrLabel;
impl PartialEq<&str> for ReplaceePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReplaceePropertyIri || *other == REPLACEE_PROPERTY_LABEL
	}
}
impl PartialEq<ReplaceePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReplaceePropertyIriOrLabel) -> bool {
		*self == ReplaceePropertyIri || *self == REPLACEE_PROPERTY_LABEL
	}
}
