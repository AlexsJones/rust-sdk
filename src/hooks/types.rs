use std::collections::HashMap;

use crate::{evaluation::EvaluationContext, providers::types::ProviderMetadata, ClientMetadata};

pub struct HookHints {
    pub map_of_hooks: HashMap<String, String>,
}

pub struct HookContext<T> {
    pub flag_key: String,
    pub default_value: T,
    pub client_meta_data: ClientMetadata,
    pub provider_meta_data: ProviderMetadata,
    pub evaluation_context: EvaluationContext,
}
