/// <https://schema.org/Blog>
pub const BLOG_IRI_HTTP: &str = "http://schema.org/Blog";
/// <https://schema.org/Blog>
pub const BLOG_IRI_HTTPS: &str = "https://schema.org/Blog";
/// <https://schema.org/Blog>
pub const BLOG_LABEL: &str = "Blog";
pub struct BlogIri;
impl PartialEq<&str> for BlogIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BLOG_IRI_HTTP || *other == BLOG_IRI_HTTPS
	}
}
impl PartialEq<BlogIri> for &str {
	fn eq(&self, other: &BlogIri) -> bool {
		*self == BLOG_IRI_HTTP || *self == BLOG_IRI_HTTPS
	}
}
pub struct BlogIriOrLabel;
impl PartialEq<&str> for BlogIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BlogIri || *other == BLOG_LABEL
	}
}
impl PartialEq<BlogIriOrLabel> for &str {
	fn eq(&self, other: &BlogIriOrLabel) -> bool {
		*self == BlogIri || *self == BLOG_LABEL
	}
}
