/// <https://schema.org/modifiedTime>
pub const MODIFIED_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/modifiedTime";
/// <https://schema.org/modifiedTime>
pub const MODIFIED_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/modifiedTime";
/// <https://schema.org/modifiedTime>
pub const MODIFIED_TIME_PROPERTY_LABEL: &str = "modifiedTime";
pub struct ModifiedTimePropertyIri;
impl PartialEq<&str> for ModifiedTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MODIFIED_TIME_PROPERTY_IRI_HTTP || *other == MODIFIED_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ModifiedTimePropertyIri> for &str {
	fn eq(&self, other: &ModifiedTimePropertyIri) -> bool {
		*self == MODIFIED_TIME_PROPERTY_IRI_HTTP || *self == MODIFIED_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct ModifiedTimePropertyIriOrLabel;
impl PartialEq<&str> for ModifiedTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ModifiedTimePropertyIri || *other == MODIFIED_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<ModifiedTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ModifiedTimePropertyIriOrLabel) -> bool {
		*self == ModifiedTimePropertyIri || *self == MODIFIED_TIME_PROPERTY_LABEL
	}
}
