/// <https://schema.org/ReturnByMail>
pub const RETURN_BY_MAIL_IRI_HTTP: &str = "http://schema.org/ReturnByMail";
/// <https://schema.org/ReturnByMail>
pub const RETURN_BY_MAIL_IRI_HTTPS: &str = "https://schema.org/ReturnByMail";
/// <https://schema.org/ReturnByMail>
pub const RETURN_BY_MAIL_LABEL: &str = "ReturnByMail";
pub struct ReturnByMailIri;
impl PartialEq<&str> for ReturnByMailIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_BY_MAIL_IRI_HTTP || *other == RETURN_BY_MAIL_IRI_HTTPS
	}
}
impl PartialEq<ReturnByMailIri> for &str {
	fn eq(&self, other: &ReturnByMailIri) -> bool {
		*self == RETURN_BY_MAIL_IRI_HTTP || *self == RETURN_BY_MAIL_IRI_HTTPS
	}
}
pub struct ReturnByMailIriOrLabel;
impl PartialEq<&str> for ReturnByMailIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnByMailIri || *other == RETURN_BY_MAIL_LABEL
	}
}
impl PartialEq<ReturnByMailIriOrLabel> for &str {
	fn eq(&self, other: &ReturnByMailIriOrLabel) -> bool {
		*self == ReturnByMailIri || *self == RETURN_BY_MAIL_LABEL
	}
}
