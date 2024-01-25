/// <https://schema.org/prepTime>
pub const PREP_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/prepTime";
/// <https://schema.org/prepTime>
pub const PREP_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/prepTime";
/// <https://schema.org/prepTime>
pub const PREP_TIME_PROPERTY_LABEL: &str = "prepTime";
pub struct PrepTimePropertyIri;
impl PartialEq<&str> for PrepTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PREP_TIME_PROPERTY_IRI_HTTP || *other == PREP_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PrepTimePropertyIri> for &str {
	fn eq(&self, other: &PrepTimePropertyIri) -> bool {
		*self == PREP_TIME_PROPERTY_IRI_HTTP || *self == PREP_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct PrepTimePropertyIriOrLabel;
impl PartialEq<&str> for PrepTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrepTimePropertyIri || *other == PREP_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<PrepTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PrepTimePropertyIriOrLabel) -> bool {
		*self == PrepTimePropertyIri || *self == PREP_TIME_PROPERTY_LABEL
	}
}
