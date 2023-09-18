use core::fmt::Debug;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::vars::ValueStream;

#[async_trait]
pub trait AsyncHint<L: PlonkParameters<D>, const D: usize>:
    'static + Debug + Clone + Send + Sync + Serialize + DeserializeOwned
{
    /// the hint function.
    async fn hint(
        &self,
        input_stream: &mut ValueStream<L, D>,
        output_stream: &mut ValueStream<L, D>,
    );

    /// a version of the hint method that owns the values.
    async fn hint_fn(&self, input_stream: ValueStream<L, D>) -> ValueStream<L, D> {
        let mut output_stream = ValueStream::new();
        self.hint(&mut input_stream.clone(), &mut output_stream)
            .await;
        output_stream
    }

    /// a unique identifier for this hint.
    ///
    /// By default, this is the type name of the hint. This function should be overwriten in case
    /// type names vary between compilation units.
    fn id() -> String {
        format!("{:?}", std::any::type_name::<Self>()).to_string()
    }
}

#[async_trait]
pub(crate) trait AnyAsyncHint<L: PlonkParameters<D>, const D: usize>:
    'static + Debug + Send + Send + Sync
{
    #[allow(unused_variables)]
    async fn hint_fn(&self, input_stream: ValueStream<L, D>) -> ValueStream<L, D> {
        unimplemented!("Implement this method")
    }
}

#[derive(Debug, Clone)]
pub struct AnyHint<H>(pub H);

#[async_trait]
impl<L: PlonkParameters<D>, H: AsyncHint<L, D>, const D: usize> AnyAsyncHint<L, D> for AnyHint<H> {
    async fn hint_fn(&self, input_stream: ValueStream<L, D>) -> ValueStream<L, D> {
        self.0.hint_fn(input_stream).await
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TestAsyncGenerator;

    #[async_trait]
    impl<L: PlonkParameters<D>, const D: usize> AsyncHint<L, D> for TestAsyncGenerator {
        async fn hint(
            &self,
            _input_stream: &mut ValueStream<L, D>,
            _output_stream: &mut ValueStream<L, D>,
        ) {
        }
    }
}
