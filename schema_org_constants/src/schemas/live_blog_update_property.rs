/// <https://schema.org/liveBlogUpdate>
pub const LIVE_BLOG_UPDATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/liveBlogUpdate";
/// <https://schema.org/liveBlogUpdate>
pub const LIVE_BLOG_UPDATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/liveBlogUpdate";
/// <https://schema.org/liveBlogUpdate>
pub const LIVE_BLOG_UPDATE_PROPERTY_LABEL: &str = "liveBlogUpdate";
pub struct LiveBlogUpdatePropertyIri;
impl PartialEq<&str> for LiveBlogUpdatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LIVE_BLOG_UPDATE_PROPERTY_IRI_HTTP
			|| *other == LIVE_BLOG_UPDATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LiveBlogUpdatePropertyIri> for &str {
	fn eq(&self, other: &LiveBlogUpdatePropertyIri) -> bool {
		*self == LIVE_BLOG_UPDATE_PROPERTY_IRI_HTTP || *self == LIVE_BLOG_UPDATE_PROPERTY_IRI_HTTPS
	}
}
pub struct LiveBlogUpdatePropertyIriOrLabel;
impl PartialEq<&str> for LiveBlogUpdatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LiveBlogUpdatePropertyIri || *other == LIVE_BLOG_UPDATE_PROPERTY_LABEL
	}
}
impl PartialEq<LiveBlogUpdatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LiveBlogUpdatePropertyIriOrLabel) -> bool {
		*self == LiveBlogUpdatePropertyIri || *self == LIVE_BLOG_UPDATE_PROPERTY_LABEL
	}
}
