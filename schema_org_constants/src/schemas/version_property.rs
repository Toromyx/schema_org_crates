/// <https://schema.org/version>
pub const VERSION_PROPERTY_IRI_HTTP: &str = "http://schema.org/version";
/// <https://schema.org/version>
pub const VERSION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/version";
/// <https://schema.org/version>
pub const VERSION_PROPERTY_LABEL: &str = "version";
pub struct VersionPropertyIri;
impl PartialEq<&str> for VersionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VERSION_PROPERTY_IRI_HTTP || *other == VERSION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VersionPropertyIri> for &str {
	fn eq(&self, other: &VersionPropertyIri) -> bool {
		*self == VERSION_PROPERTY_IRI_HTTP || *self == VERSION_PROPERTY_IRI_HTTPS
	}
}
pub struct VersionPropertyIriOrLabel;
impl PartialEq<&str> for VersionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VersionPropertyIri || *other == VERSION_PROPERTY_LABEL
	}
}
impl PartialEq<VersionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VersionPropertyIriOrLabel) -> bool {
		*self == VersionPropertyIri || *self == VERSION_PROPERTY_LABEL
	}
}
