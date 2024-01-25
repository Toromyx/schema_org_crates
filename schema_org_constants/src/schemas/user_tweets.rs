/// <https://schema.org/UserTweets>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_TWEETS_IRI_HTTP: &str = "http://schema.org/UserTweets";
/// <https://schema.org/UserTweets>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_TWEETS_IRI_HTTPS: &str = "https://schema.org/UserTweets";
/// <https://schema.org/UserTweets>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_TWEETS_LABEL: &str = "UserTweets";
pub struct UserTweetsIri;
impl PartialEq<&str> for UserTweetsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USER_TWEETS_IRI_HTTP || *other == USER_TWEETS_IRI_HTTPS
	}
}
impl PartialEq<UserTweetsIri> for &str {
	fn eq(&self, other: &UserTweetsIri) -> bool {
		*self == USER_TWEETS_IRI_HTTP || *self == USER_TWEETS_IRI_HTTPS
	}
}
pub struct UserTweetsIriOrLabel;
impl PartialEq<&str> for UserTweetsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UserTweetsIri || *other == USER_TWEETS_LABEL
	}
}
impl PartialEq<UserTweetsIriOrLabel> for &str {
	fn eq(&self, other: &UserTweetsIriOrLabel) -> bool {
		*self == UserTweetsIri || *self == USER_TWEETS_LABEL
	}
}
