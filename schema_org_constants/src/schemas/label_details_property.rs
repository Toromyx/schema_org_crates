/// <https://schema.org/labelDetails>
pub const LABEL_DETAILS_PROPERTY_IRI_HTTP: &str = "http://schema.org/labelDetails";
/// <https://schema.org/labelDetails>
pub const LABEL_DETAILS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/labelDetails";
/// <https://schema.org/labelDetails>
pub const LABEL_DETAILS_PROPERTY_LABEL: &str = "labelDetails";
pub struct LabelDetailsPropertyIri;
impl PartialEq<&str> for LabelDetailsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LABEL_DETAILS_PROPERTY_IRI_HTTP || *other == LABEL_DETAILS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LabelDetailsPropertyIri> for &str {
	fn eq(&self, other: &LabelDetailsPropertyIri) -> bool {
		*self == LABEL_DETAILS_PROPERTY_IRI_HTTP || *self == LABEL_DETAILS_PROPERTY_IRI_HTTPS
	}
}
pub struct LabelDetailsPropertyIriOrLabel;
impl PartialEq<&str> for LabelDetailsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LabelDetailsPropertyIri || *other == LABEL_DETAILS_PROPERTY_LABEL
	}
}
impl PartialEq<LabelDetailsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LabelDetailsPropertyIriOrLabel) -> bool {
		*self == LabelDetailsPropertyIri || *self == LABEL_DETAILS_PROPERTY_LABEL
	}
}
