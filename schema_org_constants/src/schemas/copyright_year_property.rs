/// <https://schema.org/copyrightYear>
pub const COPYRIGHT_YEAR_PROPERTY_IRI_HTTP: &str = "http://schema.org/copyrightYear";
/// <https://schema.org/copyrightYear>
pub const COPYRIGHT_YEAR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/copyrightYear";
/// <https://schema.org/copyrightYear>
pub const COPYRIGHT_YEAR_PROPERTY_LABEL: &str = "copyrightYear";
pub struct CopyrightYearPropertyIri;
impl PartialEq<&str> for CopyrightYearPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COPYRIGHT_YEAR_PROPERTY_IRI_HTTP || *other == COPYRIGHT_YEAR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CopyrightYearPropertyIri> for &str {
	fn eq(&self, other: &CopyrightYearPropertyIri) -> bool {
		*self == COPYRIGHT_YEAR_PROPERTY_IRI_HTTP || *self == COPYRIGHT_YEAR_PROPERTY_IRI_HTTPS
	}
}
pub struct CopyrightYearPropertyIriOrLabel;
impl PartialEq<&str> for CopyrightYearPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CopyrightYearPropertyIri || *other == COPYRIGHT_YEAR_PROPERTY_LABEL
	}
}
impl PartialEq<CopyrightYearPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CopyrightYearPropertyIriOrLabel) -> bool {
		*self == CopyrightYearPropertyIri || *self == COPYRIGHT_YEAR_PROPERTY_LABEL
	}
}
