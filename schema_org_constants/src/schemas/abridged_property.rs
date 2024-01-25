/// <https://schema.org/abridged>
pub const ABRIDGED_PROPERTY_IRI_HTTP: &str = "http://schema.org/abridged";
/// <https://schema.org/abridged>
pub const ABRIDGED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/abridged";
/// <https://schema.org/abridged>
pub const ABRIDGED_PROPERTY_LABEL: &str = "abridged";
pub struct AbridgedPropertyIri;
impl PartialEq<&str> for AbridgedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ABRIDGED_PROPERTY_IRI_HTTP || *other == ABRIDGED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AbridgedPropertyIri> for &str {
	fn eq(&self, other: &AbridgedPropertyIri) -> bool {
		*self == ABRIDGED_PROPERTY_IRI_HTTP || *self == ABRIDGED_PROPERTY_IRI_HTTPS
	}
}
pub struct AbridgedPropertyIriOrLabel;
impl PartialEq<&str> for AbridgedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AbridgedPropertyIri || *other == ABRIDGED_PROPERTY_LABEL
	}
}
impl PartialEq<AbridgedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AbridgedPropertyIriOrLabel) -> bool {
		*self == AbridgedPropertyIri || *self == ABRIDGED_PROPERTY_LABEL
	}
}
