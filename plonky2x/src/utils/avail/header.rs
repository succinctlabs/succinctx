use core::marker::PhantomData;

use codec::Encode;
use plonky2::field::extension::Extendable;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::PartitionWitness;
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult, Read, Write};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use crate::frontend::hint::simple::hint::Hint;
use crate::frontend::uint::uint32::U32Variable;
use crate::frontend::vars::ValueStream;
use crate::prelude::{ArrayVariable, Field, PlonkParameters, RichField, Witness, WitnessWrite};
use crate::utils::avail::fetch::RpcDataFetcher;
use crate::utils::avail::vars::{EncodedHeader, EncodedHeaderVariable};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderFetcherHint<const HEADER_LENGTH: usize, const NUM_HEADERS: usize> {}

impl<
        const HEADER_LENGTH: usize,
        const NUM_HEADERS: usize,
        L: PlonkParameters<D>,
        const D: usize,
    > Hint<L, D> for HeaderFetcherHint<HEADER_LENGTH, NUM_HEADERS>
{
    fn hint(&self, input_stream: &mut ValueStream<L, D>, output_stream: &mut ValueStream<L, D>) {
        let start_block = input_stream.read_value::<U32Variable>();
        let mut last_block = input_stream.read_value::<U32Variable>();
        let max_block = input_stream.read_value::<U32Variable>();

        println!(
            "HeaderFetcherHint called with params start_block: {}, last_block: {}, max_block: {}",
            start_block, last_block, max_block
        );

        last_block = last_block.min(max_block);

        let mut headers = Vec::new();
        if last_block >= start_block {
            let rt = Runtime::new().expect("failed to create tokio runtime");
            headers.extend(rt.block_on(async {
                let data_fetcher = RpcDataFetcher::new().await;
                data_fetcher
                    .get_block_headers_range(start_block, last_block)
                    .await
            }));
        }

        // We take the returned headers and pad them to the correct length to turn them into an `EncodedHeader` variable.
        let mut header_variables = Vec::new();
        for i in 0..headers.len() {
            // TODO: replace with `to_header_variable` from vars.rs
            let header = &headers[i];
            println!("in hint, got header: {:?}", header.number);
            let mut header_bytes = header.encode();
            let header_size = header_bytes.len();
            if header_size > HEADER_LENGTH {
                panic!(
                    "header size {} is greater than HEADER_LENGTH {}",
                    header_size, HEADER_LENGTH
                );
            }
            header_bytes.resize(HEADER_LENGTH, 0);
            let header_variable = EncodedHeader {
                header_bytes,
                header_size: L::Field::from_canonical_usize(header_size),
            };
            header_variables.push(header_variable);
        }

        // We must pad the rest of `header_variables` with empty headers to ensure its length is NUM_HEADERS.
        for _i in headers.len()..NUM_HEADERS {
            let header_variable = EncodedHeader {
                header_bytes: vec![0u8; HEADER_LENGTH],
                header_size: L::Field::from_canonical_usize(0),
            };
            header_variables.push(header_variable);
        }
        //println!("header_variables {:?}", header_variables);
        output_stream
            .write_value::<ArrayVariable<EncodedHeaderVariable<HEADER_LENGTH>, NUM_HEADERS>>(
                header_variables,
            );
    }
}

#[derive(Debug, Default)]
pub struct FloorDivGenerator<F: RichField + Extendable<D>, const D: usize> {
    divisor: Target,
    dividend: Target,
    quotient: Target,
    remainder: Target,
    _marker: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> FloorDivGenerator<F, D> {
    pub fn new(divisor: Target, dividend: Target, quotient: Target, remainder: Target) -> Self {
        FloorDivGenerator {
            divisor,
            dividend,
            quotient,
            remainder,
            _marker: PhantomData,
        }
    }
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for FloorDivGenerator<F, D>
{
    fn id(&self) -> String {
        "FloorDivGenerator".to_string()
    }

    fn serialize(&self, dst: &mut Vec<u8>, _common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        dst.write_target(self.divisor)?;
        dst.write_target(self.dividend)?;
        dst.write_target(self.quotient)?;
        dst.write_target(self.remainder)
    }

    fn deserialize(src: &mut Buffer, _common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        let divisor = src.read_target()?;
        let dividend = src.read_target()?;
        let quotient = src.read_target()?;
        let remainder = src.read_target()?;
        Ok(Self {
            divisor,
            dividend,
            quotient,
            remainder,
            _marker: PhantomData,
        })
    }

    fn dependencies(&self) -> Vec<Target> {
        Vec::from([self.dividend])
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let divisor = witness.get_target(self.divisor);
        let dividend = witness.get_target(self.dividend);
        let divisor_int = divisor.to_canonical_u64() as u32;
        let dividend_int = dividend.to_canonical_u64() as u32;
        let quotient = dividend_int / divisor_int;
        let remainder = dividend_int % divisor_int;
        out_buffer.set_target(self.quotient, F::from_canonical_u32(quotient));
        out_buffer.set_target(self.remainder, F::from_canonical_u32(remainder));
    }
}
