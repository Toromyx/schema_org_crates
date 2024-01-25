/// <https://schema.org/returnLabelSource>
pub const RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/returnLabelSource";
/// <https://schema.org/returnLabelSource>
pub const RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/returnLabelSource";
/// <https://schema.org/returnLabelSource>
pub const RETURN_LABEL_SOURCE_PROPERTY_LABEL: &str = "returnLabelSource";
pub struct ReturnLabelSourcePropertyIri;
impl PartialEq<&str> for ReturnLabelSourcePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTP
			|| *other == RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReturnLabelSourcePropertyIri> for &str {
	fn eq(&self, other: &ReturnLabelSourcePropertyIri) -> bool {
		*self == RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTP
			|| *self == RETURN_LABEL_SOURCE_PROPERTY_IRI_HTTPS
	}
}
pub struct ReturnLabelSourcePropertyIriOrLabel;
impl PartialEq<&str> for ReturnLabelSourcePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnLabelSourcePropertyIri || *other == RETURN_LABEL_SOURCE_PROPERTY_LABEL
	}
}
impl PartialEq<ReturnLabelSourcePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReturnLabelSourcePropertyIriOrLabel) -> bool {
		*self == ReturnLabelSourcePropertyIri || *self == RETURN_LABEL_SOURCE_PROPERTY_LABEL
	}
}
