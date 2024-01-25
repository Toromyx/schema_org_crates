/// <https://schema.org/guideline>
pub const GUIDELINE_PROPERTY_IRI_HTTP: &str = "http://schema.org/guideline";
/// <https://schema.org/guideline>
pub const GUIDELINE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/guideline";
/// <https://schema.org/guideline>
pub const GUIDELINE_PROPERTY_LABEL: &str = "guideline";
pub struct GuidelinePropertyIri;
impl PartialEq<&str> for GuidelinePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GUIDELINE_PROPERTY_IRI_HTTP || *other == GUIDELINE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GuidelinePropertyIri> for &str {
	fn eq(&self, other: &GuidelinePropertyIri) -> bool {
		*self == GUIDELINE_PROPERTY_IRI_HTTP || *self == GUIDELINE_PROPERTY_IRI_HTTPS
	}
}
pub struct GuidelinePropertyIriOrLabel;
impl PartialEq<&str> for GuidelinePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GuidelinePropertyIri || *other == GUIDELINE_PROPERTY_LABEL
	}
}
impl PartialEq<GuidelinePropertyIriOrLabel> for &str {
	fn eq(&self, other: &GuidelinePropertyIriOrLabel) -> bool {
		*self == GuidelinePropertyIri || *self == GUIDELINE_PROPERTY_LABEL
	}
}
