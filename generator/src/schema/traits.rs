use quote::{__private::TokenStream, quote};

use crate::{feature::Feature, schema::Schema};

mod base;
mod json_ld_0_15;

pub fn to_token_stream(schema: &Schema) -> TokenStream {
	let base = base::to_token_stream(schema);
	let json_ls_0_15_feature_cfg_attribute =
		Feature::Name("json-ld_0_15".to_string()).as_cfg_attribute();
	let json_ld_0_15_mod = json_ld_0_15::to_token_stream(schema);
	quote!(
		#base

		#json_ls_0_15_feature_cfg_attribute
		mod json_ld_0_15 {
			#json_ld_0_15_mod
		}
	)
}
