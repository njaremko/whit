use iota_lib_rs::pow::iss;
use iota_lib_rs::pow::kerl::Kerl;
use iota_lib_rs::pow::sponge::{Mode, Sponge, HASH_LENGTH};
use iota_lib_rs::utils::converter;
use iota_lib_rs::utils::trytes_converter;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use failure::Error;

pub fn is_file_signature_valid(
    filename: &str,
    sigature_filename: &str,
    public_key: &str,
    depth: usize,
    index: usize,
) -> Result<bool, Error> {
    let signature = digest_file(filename)?;
    validate_signature(sigature_filename, public_key, depth, index, &signature)
}

fn validate_signature(filename: &str, public_key: &str, depth: usize, index: usize, digest: &[i8]) -> Result<bool, Error>{
    let mode = Mode::CURLP81;
    let mut digests: Vec<i8> = Vec::new();
    let bundle = iss::normalized_bundle(&digest)?;
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let mut root = [0; HASH_LENGTH];
    let mut i = 0;
    for line in reader.lines().take(5) {
        if i < 3 {
            let mut line_trits = converter::trits_from_string(&line?);
            let offset = i * iss::NORMALIZED_FRAGMENT_LENGTH;
            digests.extend_from_slice(&iss::digest(
                mode,
                &bundle[offset..offset + iss::NORMALIZED_FRAGMENT_LENGTH],
                &line_trits,
            )?);
        } else {
            let mut line_trits = converter::trits_from_string(&line?);
            root = iss::get_merkle_root(
                mode,
                &iss::address(mode, &mut digests)?,
                &mut line_trits,
                0,
                index,
                depth,
            );
        }
        i += 1;
    }
    if i != 4 {
        root = iss::address(mode, &mut digests)?;
    }
    let pub_key_trits = converter::trits_from_string(public_key);

    Ok(pub_key_trits == root.to_vec())
}

fn digest_file(filename: &str) -> Result<[i8; HASH_LENGTH], Error> {
    let mut curl = Kerl::default();
    {
        let f = File::open(filename)?;
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let trits = converter::trits_from_string(&trytes_converter::to_trytes(&line?)?);
            curl.absorb(&trits);
        }
    }
    let mut trits = [0; HASH_LENGTH];
    curl.squeeze(&mut trits);
    Ok(trits)
}


