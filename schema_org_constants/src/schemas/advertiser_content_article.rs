/// <https://schema.org/AdvertiserContentArticle>
pub const ADVERTISER_CONTENT_ARTICLE_IRI_HTTP: &str = "http://schema.org/AdvertiserContentArticle";
/// <https://schema.org/AdvertiserContentArticle>
pub const ADVERTISER_CONTENT_ARTICLE_IRI_HTTPS: &str =
	"https://schema.org/AdvertiserContentArticle";
/// <https://schema.org/AdvertiserContentArticle>
pub const ADVERTISER_CONTENT_ARTICLE_LABEL: &str = "AdvertiserContentArticle";
pub struct AdvertiserContentArticleIri;
impl PartialEq<&str> for AdvertiserContentArticleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADVERTISER_CONTENT_ARTICLE_IRI_HTTP
			|| *other == ADVERTISER_CONTENT_ARTICLE_IRI_HTTPS
	}
}
impl PartialEq<AdvertiserContentArticleIri> for &str {
	fn eq(&self, other: &AdvertiserContentArticleIri) -> bool {
		*self == ADVERTISER_CONTENT_ARTICLE_IRI_HTTP
			|| *self == ADVERTISER_CONTENT_ARTICLE_IRI_HTTPS
	}
}
pub struct AdvertiserContentArticleIriOrLabel;
impl PartialEq<&str> for AdvertiserContentArticleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AdvertiserContentArticleIri || *other == ADVERTISER_CONTENT_ARTICLE_LABEL
	}
}
impl PartialEq<AdvertiserContentArticleIriOrLabel> for &str {
	fn eq(&self, other: &AdvertiserContentArticleIriOrLabel) -> bool {
		*self == AdvertiserContentArticleIri || *self == ADVERTISER_CONTENT_ARTICLE_LABEL
	}
}
