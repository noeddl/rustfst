use failure::{format_err, Fallible, ResultExt};

use serde_derive::{Deserialize, Serialize};

use crate::algorithms::{decode, encode};
use crate::fst_properties::FstProperties;
use crate::fst_traits::MutableFst;
use crate::fst_traits::TextParser;
use crate::semirings::Semiring;

use crate::tests_openfst::FstTestData;

#[derive(Serialize, Deserialize, Debug)]
pub struct EncodeOperationResult {
    encode_labels: bool,
    encode_weights: bool,
    result: String,
}

pub struct EncodeTestData<F>
where
    F: TextParser,
    F::W: Semiring<Type = f32>,
{
    pub encode_labels: bool,
    pub encode_weights: bool,
    pub result: F,
}

impl EncodeOperationResult {
    pub fn parse<F>(&self) -> EncodeTestData<F>
    where
        F: TextParser,
        F::W: Semiring<Type = f32>,
    {
        EncodeTestData {
            encode_weights: self.encode_weights,
            encode_labels: self.encode_labels,
            result: F::from_text_string(self.result.as_str()).unwrap(),
        }
    }
}

pub fn test_encode_decode<F>(test_data: &FstTestData<F>) -> Fallible<()>
where
    F: TextParser + MutableFst,
    F::W: Semiring<Type = f32>,
{
    for encode_test_data in &test_data.encode_decode {
        let mut fst_encoded = test_data.raw.clone();
        let encode_table = encode(&mut fst_encoded, encode_test_data.encode_labels, encode_test_data.encode_weights)
            .with_context(|_| format_err!(
            "Error when running test_encode_decode with parameters encode_labels={:?} and encode_weights={:?}.",
            encode_test_data.encode_labels, encode_test_data.encode_weights))?;
        decode(&mut fst_encoded, encode_table)?;
        assert_eq!(
            encode_test_data.result,
            fst_encoded,
            "{}",
            error_message_fst!(
                encode_test_data.result,
                fst_encoded,
                format!(
                    "Encode/Decode with encode_labels={:?} and encode_weights={:?}",
                    encode_test_data.encode_labels, encode_test_data.encode_weights
                )
            )
        );
    }
    Ok(())
}

pub fn test_encode<F>(test_data: &FstTestData<F>) -> Fallible<()>
where
    F: TextParser + MutableFst,
    F::W: Semiring<Type = f32>,
{
    for encode_test_data in &test_data.encode {
        let mut fst_encoded = test_data.raw.clone();
        encode(&mut fst_encoded, encode_test_data.encode_labels, encode_test_data.encode_weights)
            .with_context(|_| format_err!(
            "Error when running test_encode with parameters encode_labels={:?} and encode_weights={:?}.",
            encode_test_data.encode_labels, encode_test_data.encode_weights))?;
        if encode_test_data.encode_labels {
            assert!(fst_encoded.properties()?.contains(FstProperties::ACCEPTOR));
        }
        if encode_test_data.encode_weights {
            assert!(fst_encoded
                .properties()?
                .contains(FstProperties::UNWEIGHTED));
        }
        assert_eq!(
            encode_test_data.result,
            fst_encoded,
            "{}",
            error_message_fst!(encode_test_data.result, fst_encoded, "Encode")
        );
    }
    Ok(())
}
