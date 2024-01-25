/// <https://schema.org/SuspendAction>
pub const SUSPEND_ACTION_IRI_HTTP: &str = "http://schema.org/SuspendAction";
/// <https://schema.org/SuspendAction>
pub const SUSPEND_ACTION_IRI_HTTPS: &str = "https://schema.org/SuspendAction";
/// <https://schema.org/SuspendAction>
pub const SUSPEND_ACTION_LABEL: &str = "SuspendAction";
pub struct SuspendActionIri;
impl PartialEq<&str> for SuspendActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUSPEND_ACTION_IRI_HTTP || *other == SUSPEND_ACTION_IRI_HTTPS
	}
}
impl PartialEq<SuspendActionIri> for &str {
	fn eq(&self, other: &SuspendActionIri) -> bool {
		*self == SUSPEND_ACTION_IRI_HTTP || *self == SUSPEND_ACTION_IRI_HTTPS
	}
}
pub struct SuspendActionIriOrLabel;
impl PartialEq<&str> for SuspendActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SuspendActionIri || *other == SUSPEND_ACTION_LABEL
	}
}
impl PartialEq<SuspendActionIriOrLabel> for &str {
	fn eq(&self, other: &SuspendActionIriOrLabel) -> bool {
		*self == SuspendActionIri || *self == SUSPEND_ACTION_LABEL
	}
}
