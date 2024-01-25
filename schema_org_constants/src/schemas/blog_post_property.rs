/// <https://schema.org/blogPost>
pub const BLOG_POST_PROPERTY_IRI_HTTP: &str = "http://schema.org/blogPost";
/// <https://schema.org/blogPost>
pub const BLOG_POST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/blogPost";
/// <https://schema.org/blogPost>
pub const BLOG_POST_PROPERTY_LABEL: &str = "blogPost";
pub struct BlogPostPropertyIri;
impl PartialEq<&str> for BlogPostPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BLOG_POST_PROPERTY_IRI_HTTP || *other == BLOG_POST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BlogPostPropertyIri> for &str {
	fn eq(&self, other: &BlogPostPropertyIri) -> bool {
		*self == BLOG_POST_PROPERTY_IRI_HTTP || *self == BLOG_POST_PROPERTY_IRI_HTTPS
	}
}
pub struct BlogPostPropertyIriOrLabel;
impl PartialEq<&str> for BlogPostPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BlogPostPropertyIri || *other == BLOG_POST_PROPERTY_LABEL
	}
}
impl PartialEq<BlogPostPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BlogPostPropertyIriOrLabel) -> bool {
		*self == BlogPostPropertyIri || *self == BLOG_POST_PROPERTY_LABEL
	}
}
