/// <https://schema.org/PostOffice>
pub const POST_OFFICE_IRI_HTTP: &str = "http://schema.org/PostOffice";
/// <https://schema.org/PostOffice>
pub const POST_OFFICE_IRI_HTTPS: &str = "https://schema.org/PostOffice";
/// <https://schema.org/PostOffice>
pub const POST_OFFICE_LABEL: &str = "PostOffice";
pub struct PostOfficeIri;
impl PartialEq<&str> for PostOfficeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POST_OFFICE_IRI_HTTP || *other == POST_OFFICE_IRI_HTTPS
	}
}
impl PartialEq<PostOfficeIri> for &str {
	fn eq(&self, other: &PostOfficeIri) -> bool {
		*self == POST_OFFICE_IRI_HTTP || *self == POST_OFFICE_IRI_HTTPS
	}
}
pub struct PostOfficeIriOrLabel;
impl PartialEq<&str> for PostOfficeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PostOfficeIri || *other == POST_OFFICE_LABEL
	}
}
impl PartialEq<PostOfficeIriOrLabel> for &str {
	fn eq(&self, other: &PostOfficeIriOrLabel) -> bool {
		*self == PostOfficeIri || *self == POST_OFFICE_LABEL
	}
}
