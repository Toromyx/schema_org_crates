/// <https://schema.org/SocialMediaPosting>
pub const SOCIAL_MEDIA_POSTING_IRI_HTTP: &str = "http://schema.org/SocialMediaPosting";
/// <https://schema.org/SocialMediaPosting>
pub const SOCIAL_MEDIA_POSTING_IRI_HTTPS: &str = "https://schema.org/SocialMediaPosting";
/// <https://schema.org/SocialMediaPosting>
pub const SOCIAL_MEDIA_POSTING_LABEL: &str = "SocialMediaPosting";
pub struct SocialMediaPostingIri;
impl PartialEq<&str> for SocialMediaPostingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SOCIAL_MEDIA_POSTING_IRI_HTTP || *other == SOCIAL_MEDIA_POSTING_IRI_HTTPS
	}
}
impl PartialEq<SocialMediaPostingIri> for &str {
	fn eq(&self, other: &SocialMediaPostingIri) -> bool {
		*self == SOCIAL_MEDIA_POSTING_IRI_HTTP || *self == SOCIAL_MEDIA_POSTING_IRI_HTTPS
	}
}
pub struct SocialMediaPostingIriOrLabel;
impl PartialEq<&str> for SocialMediaPostingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SocialMediaPostingIri || *other == SOCIAL_MEDIA_POSTING_LABEL
	}
}
impl PartialEq<SocialMediaPostingIriOrLabel> for &str {
	fn eq(&self, other: &SocialMediaPostingIriOrLabel) -> bool {
		*self == SocialMediaPostingIri || *self == SOCIAL_MEDIA_POSTING_LABEL
	}
}
