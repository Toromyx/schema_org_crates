/// <https://schema.org/blogPosts>
#[deprecated = "This schema is superseded by <https://schema.org/blogPost>."]
pub const BLOG_POSTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/blogPosts";
/// <https://schema.org/blogPosts>
#[deprecated = "This schema is superseded by <https://schema.org/blogPost>."]
pub const BLOG_POSTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/blogPosts";
/// <https://schema.org/blogPosts>
#[deprecated = "This schema is superseded by <https://schema.org/blogPost>."]
pub const BLOG_POSTS_PROPERTY_LABEL: &str = "blogPosts";
pub struct BlogPostsPropertyIri;
impl PartialEq<&str> for BlogPostsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BLOG_POSTS_PROPERTY_IRI_HTTP || *other == BLOG_POSTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BlogPostsPropertyIri> for &str {
	fn eq(&self, other: &BlogPostsPropertyIri) -> bool {
		*self == BLOG_POSTS_PROPERTY_IRI_HTTP || *self == BLOG_POSTS_PROPERTY_IRI_HTTPS
	}
}
pub struct BlogPostsPropertyIriOrLabel;
impl PartialEq<&str> for BlogPostsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BlogPostsPropertyIri || *other == BLOG_POSTS_PROPERTY_LABEL
	}
}
impl PartialEq<BlogPostsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BlogPostsPropertyIriOrLabel) -> bool {
		*self == BlogPostsPropertyIri || *self == BLOG_POSTS_PROPERTY_LABEL
	}
}
