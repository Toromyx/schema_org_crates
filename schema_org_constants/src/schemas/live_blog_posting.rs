/// <https://schema.org/LiveBlogPosting>
pub const LIVE_BLOG_POSTING_IRI_HTTP: &str = "http://schema.org/LiveBlogPosting";
/// <https://schema.org/LiveBlogPosting>
pub const LIVE_BLOG_POSTING_IRI_HTTPS: &str = "https://schema.org/LiveBlogPosting";
/// <https://schema.org/LiveBlogPosting>
pub const LIVE_BLOG_POSTING_LABEL: &str = "LiveBlogPosting";
pub struct LiveBlogPostingIri;
impl PartialEq<&str> for LiveBlogPostingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LIVE_BLOG_POSTING_IRI_HTTP || *other == LIVE_BLOG_POSTING_IRI_HTTPS
	}
}
impl PartialEq<LiveBlogPostingIri> for &str {
	fn eq(&self, other: &LiveBlogPostingIri) -> bool {
		*self == LIVE_BLOG_POSTING_IRI_HTTP || *self == LIVE_BLOG_POSTING_IRI_HTTPS
	}
}
pub struct LiveBlogPostingIriOrLabel;
impl PartialEq<&str> for LiveBlogPostingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LiveBlogPostingIri || *other == LIVE_BLOG_POSTING_LABEL
	}
}
impl PartialEq<LiveBlogPostingIriOrLabel> for &str {
	fn eq(&self, other: &LiveBlogPostingIriOrLabel) -> bool {
		*self == LiveBlogPostingIri || *self == LIVE_BLOG_POSTING_LABEL
	}
}
