use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TokenExpiresResponse{
    #[schema(example = "1735689600")]
    pub expires_at: usize
}