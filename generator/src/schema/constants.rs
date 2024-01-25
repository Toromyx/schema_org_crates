use std::str::FromStr;

use convert_case::{Case, Casing};
use quote::{__private::TokenStream, quote};

use crate::{deprecated_attribute::DeprecatedAttribute, doc_lines::DocLines, schema::Schema};

fn const_base(schema: &Schema) -> String {
	schema.type_name().to_case(Case::ScreamingSnake)
}

pub fn const_iri_http(schema: &Schema) -> TokenStream {
	TokenStream::from_str(&format!("{}_IRI_HTTP", const_base(schema))).unwrap()
}

pub fn const_iri_https(schema: &Schema) -> TokenStream {
	TokenStream::from_str(&format!("{}_IRI_HTTPS", const_base(schema))).unwrap()
}

pub fn const_label(schema: &Schema) -> TokenStream {
	TokenStream::from_str(&format!("{}_LABEL", const_base(schema))).unwrap()
}

fn struct_base(schema: &Schema) -> String {
	schema.type_name().to_case(Case::UpperCamel)
}

pub fn struct_iri(schema: &Schema) -> TokenStream {
	TokenStream::from_str(&format!("{}Iri", struct_base(schema))).unwrap()
}

pub fn struct_iri_or_label(schema: &Schema) -> TokenStream {
	TokenStream::from_str(&format!("{}IriOrLabel", struct_base(schema))).unwrap()
}

pub fn to_token_stream(schema: &Schema) -> TokenStream {
	let doc_lines = schema.doc_lines_token_stream();
	let deprecated_attribute = schema.deprecated_attribute();
	let const_iri_http = const_iri_http(schema);
	let const_iri_https = const_iri_https(schema);
	let const_label = const_label(schema);
	let iri_http = schema.iri.replacen("https", "http", 1);
	let iri_https = &schema.iri;
	let label = &schema.name;
	let struct_iri = struct_iri(schema);
	let struct_iri_or_label = struct_iri_or_label(schema);
	quote!(
		#doc_lines
		#deprecated_attribute
		pub const #const_iri_http: &str = #iri_http;
		#doc_lines
		#deprecated_attribute
		pub const #const_iri_https: &str = #iri_https;
		#doc_lines
		#deprecated_attribute
		pub const #const_label: &str = #label;

		pub struct #struct_iri;

		impl PartialEq<&str> for #struct_iri {
			fn eq(&self, other: &&str) -> bool {
				*other == #const_iri_http || *other == #const_iri_https
			}
		}

		impl PartialEq<#struct_iri> for &str {
			fn eq(&self, other: &#struct_iri) -> bool {
				*self == #const_iri_http || *self == #const_iri_https
			}
		}

		pub struct #struct_iri_or_label;

		impl PartialEq<&str> for #struct_iri_or_label {
			fn eq(&self, other: &&str) -> bool {
				*other == #struct_iri || *other == #const_label
			}
		}

		impl PartialEq<#struct_iri_or_label> for &str {
			fn eq(&self, other: &#struct_iri_or_label) -> bool {
				*self == #struct_iri || *self == #const_label
			}
		}
	)
}
