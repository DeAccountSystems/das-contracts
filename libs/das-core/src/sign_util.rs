use super::{
    constants::{SignType, SECP_SIGNATURE_SIZE},
    error::Error,
    util,
};
use crate::constants::ScriptType;
use alloc::{vec, vec::Vec};
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{packed::*, prelude::*},
    error::SysError,
    high_level, syscalls,
};

fn find_input_size() -> Result<usize, Error> {
    let mut i = 1;
    loop {
        let mut buf = [0u8; 1];
        match syscalls::load_input(&mut buf, 0, i, Source::Input) {
            Ok(_) => {
                // continue counting ...
            }
            Err(SysError::LengthNotEnough(_)) => {
                // continue counting ...
            }
            Err(SysError::IndexOutOfBound) => {
                break;
            }
            Err(err) => {
                return Err(Error::from(err));
            }
        }

        i += 1;
    }

    Ok(i)
}

pub fn calc_digest_by_lock(sign_type: SignType, script: ScriptReader) -> Result<([u8; 32], Vec<u8>), Error> {
    let input_group_idxs = util::find_cells_by_script(ScriptType::Lock, script, Source::Input)?;
    let ret = calc_digest_by_input_group(sign_type, input_group_idxs)?;

    Ok(ret)
}

pub fn calc_digest_by_input_group(
    sign_type: SignType,
    input_group_idxs: Vec<usize>,
) -> Result<([u8; 32], Vec<u8>), Error> {
    debug!(
        "Calculate digest by input group ... (sign_type: {:?}, input_group: {:?})",
        sign_type, input_group_idxs
    );

    let init_witness_idx = input_group_idxs[0];
    let witness_bytes = util::load_witnesses(init_witness_idx)?;
    let init_witness = WitnessArgs::from_slice(&witness_bytes).map_err(|_| {
        warn!(
            "  inputs[{}] Witness can not be decoded as WitnessArgs.(data: 0x{})",
            init_witness_idx,
            util::hex_string(&witness_bytes)
        );
        Error::WitnessArgsDecodingError
    })?;

    // Extract signatures and reset witness_args to empty status for calculation of digest.
    match init_witness.as_reader().lock().to_opt() {
        None => Err(Error::WitnessArgsInvalid),
        Some(witness_args_lock) => {
            debug!(
                "  inputs[{}] Generating digest ... (sign_type: {:?}, witness_args.lock: 0x{}",
                init_witness_idx,
                sign_type,
                util::first_n_bytes_to_hex(witness_args_lock.raw_data(), 20)
            );

            let signatures;
            let witness_args_lock_without_sig = match sign_type {
                SignType::Secp256k1Blake160MultiSigAll => {
                    let bytes = witness_args_lock.raw_data();
                    let _reserved_byte = bytes[0];
                    let _require_first_n = bytes[1];
                    let _threshold = bytes[2];
                    let signature_addresses_len = bytes[3];
                    let slice_point = (4 + 20 * signature_addresses_len) as usize;

                    signatures = bytes[slice_point..].to_vec();

                    debug!(
                        "  inputs[{}] Slice WitnessArgs.lock at {} .(header: 0x{}, args: 0x{}, signatures: {})",
                        init_witness_idx,
                        slice_point,
                        util::hex_string(&bytes[..4]),
                        util::hex_string(&bytes[4..slice_point]),
                        util::first_n_bytes_to_hex(&signatures, 10)
                    );

                    let mut data = bytes[..slice_point].to_vec();
                    let padding = vec![0u8; SECP_SIGNATURE_SIZE * slice_point];
                    data.extend_from_slice(&padding);

                    data
                }
                SignType::Secp256k1Blake160SignhashAll | SignType::EIP712Custom => {
                    signatures = witness_args_lock.raw_data().to_vec();
                    vec![0u8; SECP_SIGNATURE_SIZE]
                }
            };

            let lock = BytesOpt::new_builder()
                .set(Some(witness_args_lock_without_sig.pack()))
                .build();
            let mut witness_args_builder = init_witness.as_builder();
            witness_args_builder = witness_args_builder.lock(lock);

            let witness_args_without_sig = witness_args_builder.build();
            let tx_hash = high_level::load_tx_hash().map_err(|_| Error::ItemMissing)?;

            let mut blake2b = util::new_blake2b();
            blake2b.update(&tx_hash);
            blake2b.update(&(witness_args_without_sig.as_bytes().len() as u64).to_le_bytes());
            blake2b.update(&witness_args_without_sig.as_bytes());
            for idx in input_group_idxs.iter().skip(1).cloned() {
                let other_witness_bytes = util::load_witnesses(idx)?;
                blake2b.update(&(other_witness_bytes.len() as u64).to_le_bytes());
                blake2b.update(&other_witness_bytes);
            }
            let mut i = find_input_size()?;
            loop {
                let ret = util::load_witnesses(i);
                match ret {
                    Ok(outter_witness_bytes) => {
                        blake2b.update(&(outter_witness_bytes.len() as u64).to_le_bytes());
                        blake2b.update(&outter_witness_bytes);
                    }
                    Err(Error::IndexOutOfBound) => {
                        break;
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }

                i += 1;
            }
            let mut digest = [0u8; 32];
            blake2b.finalize(&mut digest);

            debug!(
                "  inputs[{}] Generate digest: 0x{}",
                init_witness_idx,
                util::hex_string(&digest)
            );

            Ok((digest, signatures))
        }
    }
}