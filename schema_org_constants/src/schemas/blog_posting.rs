/// <https://schema.org/BlogPosting>
pub const BLOG_POSTING_IRI_HTTP: &str = "http://schema.org/BlogPosting";
/// <https://schema.org/BlogPosting>
pub const BLOG_POSTING_IRI_HTTPS: &str = "https://schema.org/BlogPosting";
/// <https://schema.org/BlogPosting>
pub const BLOG_POSTING_LABEL: &str = "BlogPosting";
pub struct BlogPostingIri;
impl PartialEq<&str> for BlogPostingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BLOG_POSTING_IRI_HTTP || *other == BLOG_POSTING_IRI_HTTPS
	}
}
impl PartialEq<BlogPostingIri> for &str {
	fn eq(&self, other: &BlogPostingIri) -> bool {
		*self == BLOG_POSTING_IRI_HTTP || *self == BLOG_POSTING_IRI_HTTPS
	}
}
pub struct BlogPostingIriOrLabel;
impl PartialEq<&str> for BlogPostingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BlogPostingIri || *other == BLOG_POSTING_LABEL
	}
}
impl PartialEq<BlogPostingIriOrLabel> for &str {
	fn eq(&self, other: &BlogPostingIriOrLabel) -> bool {
		*self == BlogPostingIri || *self == BLOG_POSTING_LABEL
	}
}
