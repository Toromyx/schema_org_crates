/// <https://schema.org/postOfficeBoxNumber>
pub const POST_OFFICE_BOX_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/postOfficeBoxNumber";
/// <https://schema.org/postOfficeBoxNumber>
pub const POST_OFFICE_BOX_NUMBER_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/postOfficeBoxNumber";
/// <https://schema.org/postOfficeBoxNumber>
pub const POST_OFFICE_BOX_NUMBER_PROPERTY_LABEL: &str = "postOfficeBoxNumber";
pub struct PostOfficeBoxNumberPropertyIri;
impl PartialEq<&str> for PostOfficeBoxNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POST_OFFICE_BOX_NUMBER_PROPERTY_IRI_HTTP
			|| *other == POST_OFFICE_BOX_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PostOfficeBoxNumberPropertyIri> for &str {
	fn eq(&self, other: &PostOfficeBoxNumberPropertyIri) -> bool {
		*self == POST_OFFICE_BOX_NUMBER_PROPERTY_IRI_HTTP
			|| *self == POST_OFFICE_BOX_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct PostOfficeBoxNumberPropertyIriOrLabel;
impl PartialEq<&str> for PostOfficeBoxNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PostOfficeBoxNumberPropertyIri || *other == POST_OFFICE_BOX_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<PostOfficeBoxNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PostOfficeBoxNumberPropertyIriOrLabel) -> bool {
		*self == PostOfficeBoxNumberPropertyIri || *self == POST_OFFICE_BOX_NUMBER_PROPERTY_LABEL
	}
}
