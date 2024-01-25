/// <https://schema.org/ReturnLabelInBox>
pub const RETURN_LABEL_IN_BOX_IRI_HTTP: &str = "http://schema.org/ReturnLabelInBox";
/// <https://schema.org/ReturnLabelInBox>
pub const RETURN_LABEL_IN_BOX_IRI_HTTPS: &str = "https://schema.org/ReturnLabelInBox";
/// <https://schema.org/ReturnLabelInBox>
pub const RETURN_LABEL_IN_BOX_LABEL: &str = "ReturnLabelInBox";
pub struct ReturnLabelInBoxIri;
impl PartialEq<&str> for ReturnLabelInBoxIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_LABEL_IN_BOX_IRI_HTTP || *other == RETURN_LABEL_IN_BOX_IRI_HTTPS
	}
}
impl PartialEq<ReturnLabelInBoxIri> for &str {
	fn eq(&self, other: &ReturnLabelInBoxIri) -> bool {
		*self == RETURN_LABEL_IN_BOX_IRI_HTTP || *self == RETURN_LABEL_IN_BOX_IRI_HTTPS
	}
}
pub struct ReturnLabelInBoxIriOrLabel;
impl PartialEq<&str> for ReturnLabelInBoxIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnLabelInBoxIri || *other == RETURN_LABEL_IN_BOX_LABEL
	}
}
impl PartialEq<ReturnLabelInBoxIriOrLabel> for &str {
	fn eq(&self, other: &ReturnLabelInBoxIriOrLabel) -> bool {
		*self == ReturnLabelInBoxIri || *self == RETURN_LABEL_IN_BOX_LABEL
	}
}
