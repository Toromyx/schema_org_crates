/// <https://schema.org/guidelineDate>
pub const GUIDELINE_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/guidelineDate";
/// <https://schema.org/guidelineDate>
pub const GUIDELINE_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/guidelineDate";
/// <https://schema.org/guidelineDate>
pub const GUIDELINE_DATE_PROPERTY_LABEL: &str = "guidelineDate";
pub struct GuidelineDatePropertyIri;
impl PartialEq<&str> for GuidelineDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GUIDELINE_DATE_PROPERTY_IRI_HTTP || *other == GUIDELINE_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GuidelineDatePropertyIri> for &str {
	fn eq(&self, other: &GuidelineDatePropertyIri) -> bool {
		*self == GUIDELINE_DATE_PROPERTY_IRI_HTTP || *self == GUIDELINE_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct GuidelineDatePropertyIriOrLabel;
impl PartialEq<&str> for GuidelineDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GuidelineDatePropertyIri || *other == GUIDELINE_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<GuidelineDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &GuidelineDatePropertyIriOrLabel) -> bool {
		*self == GuidelineDatePropertyIri || *self == GUIDELINE_DATE_PROPERTY_LABEL
	}
}
