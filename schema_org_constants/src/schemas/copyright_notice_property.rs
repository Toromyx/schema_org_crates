/// <https://schema.org/copyrightNotice>
pub const COPYRIGHT_NOTICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/copyrightNotice";
/// <https://schema.org/copyrightNotice>
pub const COPYRIGHT_NOTICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/copyrightNotice";
/// <https://schema.org/copyrightNotice>
pub const COPYRIGHT_NOTICE_PROPERTY_LABEL: &str = "copyrightNotice";
pub struct CopyrightNoticePropertyIri;
impl PartialEq<&str> for CopyrightNoticePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COPYRIGHT_NOTICE_PROPERTY_IRI_HTTP
			|| *other == COPYRIGHT_NOTICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CopyrightNoticePropertyIri> for &str {
	fn eq(&self, other: &CopyrightNoticePropertyIri) -> bool {
		*self == COPYRIGHT_NOTICE_PROPERTY_IRI_HTTP || *self == COPYRIGHT_NOTICE_PROPERTY_IRI_HTTPS
	}
}
pub struct CopyrightNoticePropertyIriOrLabel;
impl PartialEq<&str> for CopyrightNoticePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CopyrightNoticePropertyIri || *other == COPYRIGHT_NOTICE_PROPERTY_LABEL
	}
}
impl PartialEq<CopyrightNoticePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CopyrightNoticePropertyIriOrLabel) -> bool {
		*self == CopyrightNoticePropertyIri || *self == COPYRIGHT_NOTICE_PROPERTY_LABEL
	}
}
