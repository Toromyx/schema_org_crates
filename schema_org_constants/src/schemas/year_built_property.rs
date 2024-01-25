/// <https://schema.org/yearBuilt>
pub const YEAR_BUILT_PROPERTY_IRI_HTTP: &str = "http://schema.org/yearBuilt";
/// <https://schema.org/yearBuilt>
pub const YEAR_BUILT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/yearBuilt";
/// <https://schema.org/yearBuilt>
pub const YEAR_BUILT_PROPERTY_LABEL: &str = "yearBuilt";
pub struct YearBuiltPropertyIri;
impl PartialEq<&str> for YearBuiltPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == YEAR_BUILT_PROPERTY_IRI_HTTP || *other == YEAR_BUILT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<YearBuiltPropertyIri> for &str {
	fn eq(&self, other: &YearBuiltPropertyIri) -> bool {
		*self == YEAR_BUILT_PROPERTY_IRI_HTTP || *self == YEAR_BUILT_PROPERTY_IRI_HTTPS
	}
}
pub struct YearBuiltPropertyIriOrLabel;
impl PartialEq<&str> for YearBuiltPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == YearBuiltPropertyIri || *other == YEAR_BUILT_PROPERTY_LABEL
	}
}
impl PartialEq<YearBuiltPropertyIriOrLabel> for &str {
	fn eq(&self, other: &YearBuiltPropertyIriOrLabel) -> bool {
		*self == YearBuiltPropertyIri || *self == YEAR_BUILT_PROPERTY_LABEL
	}
}
