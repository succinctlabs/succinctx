use core::fmt::Debug;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::backend::circuit::PlonkParameters;
use crate::frontend::vars::ValueStream;

/// An asynchronous hint.
///
/// This type of hint can used to perform asynchronous operations during witness generation.
///
/// ## Example
/// The following example shows how to use an asynchronous hint that gets an input byte, sleeps
/// for the number of miliseconds specified by the byte, and then outputs the byte.
/// ```
/// # use async_trait::async_trait;
/// # use serde::{Deserialize, Serialize};
/// # use tokio::time::{sleep, Duration};
//  # use plonky2x::frontend::vars::ValueStream;
/// # use plonky2x::frontend::vars::ValueStream;
/// # use plonky2x::prelude::*;
/// # use plonky2x::frontend::hint::asynchronous::hint::AsyncHint;
///
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// struct SleepHint;
///
/// #[async_trait]
/// impl<L: PlonkParameters<D>, const D: usize> AsyncHint<L, D> for SleepHint {
///    async fn hint(
///        &self,
///        input_stream: &mut ValueStream<L, D>,
///        output_stream: &mut ValueStream<L, D>,
///     ) {
///         let time = input_stream.read_value::<ByteVariable>();
///         sleep(Duration::from_millis(time.into())).await;
///         output_stream.write_value::<ByteVariable>(time);
///     }
/// }
/// ```
#[async_trait]
pub trait AsyncHint<L: PlonkParameters<D>, const D: usize>:
    'static + Debug + Clone + Send + Sync + Serialize + DeserializeOwned
{
    /// The hint function.
    async fn hint(
        &self,
        input_stream: &mut ValueStream<L, D>,
        output_stream: &mut ValueStream<L, D>,
    );

    /// A version of the hint function that owns the input stream and returns the output stream.
    ///
    /// Only one of `hint` or `hint_fn` needs to be implemented. By default, `hint_fn` calls `hint`.
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
        std::any::type_name::<Self>().to_string()
    }
}

/// A version of `AsyncHint` that that is [object safe][1] and can be used as a trait object.
///
/// [1]: https://doc.rust-lang.org/reference/items/traits.html#object-safety
#[async_trait]
pub(crate) trait AnyAsyncHint<L: PlonkParameters<D>, const D: usize>:
    'static + Debug + Send + Send + Sync
{
    #[allow(unused_variables)]
    async fn hint_fn(&self, input_stream: ValueStream<L, D>) -> ValueStream<L, D> {
        unimplemented!("Implement this method")
    }
}

/// A wrapper around an asynchronous hint that implements `AnyAsyncHint`.
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
