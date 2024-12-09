use crate::types::{Context, MyResult};

#[allow(dead_code)]
pub trait Job {
    fn name(&self) -> &str;

    async fn run(&self, ctx: &Context, metadata: &serde_json::Value) -> MyResult<()>;
}
