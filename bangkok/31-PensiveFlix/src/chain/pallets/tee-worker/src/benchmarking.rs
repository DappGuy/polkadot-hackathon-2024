#![cfg(feature = "runtime-benchmarks")]

use super::*;
use sp_core::crypto::DEV_PHRASE;
use scale_info::prelude::format;
use pfx_types::AttestationProvider;
use sp_io::crypto::{sr25519_generate, sr25519_sign};
use sp_core::crypto::KeyTypeId;
// use crate::{Pallet as TeeWorker};
// use codec::{alloc::string::ToString, Decode};
// pub use frame_benchmarking::{
// 	account, benchmarks, impl_benchmark_test_suite, whitelist_account, whitelisted_caller,
// };
// use frame_support::{
// 	traits::{Currency, CurrencyToVote, Imbalance},
// };
// use frame_system::RawOrigin;

// pub struct Pallet<T: Config>(TeeWorker<T>);
// pub trait Config:
// 	crate::Config + pallet_cess_staking::Config
// {
// }

const USER_SEED: u32 = 999666;
const KEY_ID: KeyTypeId = KeyTypeId(*b"xyjj");
// const PODR2_PBK: [u8; 270] = [48, 130, 1, 10, 2, 130, 1, 1, 0, 151, 247, 38, 216, 188, 81, 0, 64, 239, 101, 82, 181, 134, 30, 84, 171, 120, 21, 39, 196, 216, 82, 17, 10, 225, 78, 91, 35, 182, 8, 50, 1, 164, 235, 206, 201, 233, 223, 174, 125, 138, 51, 70, 14, 20, 198, 52, 199, 9, 142, 65, 183, 251, 134, 22, 114, 242, 205, 169, 28, 115, 213, 82, 82, 170, 63, 57, 24, 59, 136, 162, 231, 123, 234, 187, 175, 244, 95, 232, 254, 130, 17, 46, 163, 201, 93, 244, 240, 179, 30, 43, 221, 67, 226, 161, 22, 53, 161, 77, 221, 124, 48, 21, 62, 181, 108, 63, 10, 7, 160, 92, 170, 151, 111, 51, 112, 0, 183, 168, 219, 78, 105, 29, 47, 84, 34, 159, 17, 118, 194, 115, 4, 208, 110, 56, 76, 22, 202, 200, 104, 210, 97, 102, 183, 86, 63, 108, 50, 179, 21, 236, 147, 57, 142, 76, 67, 243, 235, 162, 202, 147, 163, 148, 237, 7, 229, 122, 250, 75, 249, 99, 242, 157, 34, 136, 175, 74, 140, 73, 145, 9, 131, 56, 88, 249, 175, 17, 26, 93, 72, 15, 50, 85, 230, 91, 124, 235, 26, 104, 200, 161, 160, 137, 249, 3, 49, 7, 119, 207, 100, 127, 108, 215, 251, 214, 228, 180, 221, 223, 12, 163, 66, 214, 185, 100, 84, 65, 6, 71, 240, 45, 247, 107, 193, 238, 68, 67, 92, 161, 223, 1, 43, 150, 137, 218, 221, 213, 232, 36, 5, 121, 32, 59, 129, 75, 5, 2, 3, 1, 0, 1];
// const PEER_ID: [u8; 38] = [0, 36, 8, 1, 18, 32, 12, 50, 93, 198, 152, 25, 126, 164, 106, 27, 26, 219, 151, 207, 191, 72, 133, 39, 20, 109, 117, 187, 165, 127, 101, 12, 167, 175, 255, 138, 213, 34];
// const NODE_PUBLIC_KEY: NodePublicKey = sp_core::ed25519::Public([12, 50, 93, 198, 152, 25, 126, 164, 106, 27, 26, 219, 151, 207, 191, 72, 133, 39, 20, 109, 117, 187, 165, 127, 101, 12, 167, 175, 255, 138, 213, 34]);
pub fn generate_workers<T: Config>() -> DispatchResult {
    let (stash_account, _) = pallet_cess_staking::testing_utils::create_stash_controller::<T>(USER_SEED, 100, pallet_cess_staking::RewardDestination::Staked)?;
    let pubkey = sr25519_generate(KEY_ID, None);
    <MasterPubkey<T>>::put(pubkey);
    let worker_info = WorkerInfo::<AccountOf<T>> {
        pubkey,
        ecdh_pubkey: pubkey,
        version: 0,
        last_updated: 1,
        stash_account: Some(stash_account),
        attestation_provider: Some(AttestationProvider::Root),
        confidence_level: 128u8,
        features: vec![1, 4],
    };
    <Workers<T>>::insert(&pubkey, worker_info);
    ValidationTypeList::<T>::mutate(|puk_list| -> DispatchResult {
        puk_list
            .try_push(pubkey)
            .map_err(|_| Error::<T>::BoundedVecError)?;
        Ok(())
    })?;

    Ok(())
}

pub fn get_pubkey<T: Config>() -> WorkerPublicKey {
    let pubkey = <MasterPubkey<T>>::get().unwrap();

    pubkey
}

pub fn sign_message<T: Config>(msg: &[u8]) -> sp_core::sr25519::Signature {
    let pubkey = <MasterPubkey<T>>::get().unwrap();
    sr25519_sign(
        KEY_ID,
        &pubkey,
        msg
    ).unwrap()
}

// pub fn get_report() -> SgxAttestationReport {
//     SgxAttestationReport {
//         report_json_raw: [123, 34, 105, 100, 34, 58, 34, 49, 52, 53, 48, 49, 48, 48, 56, 52, 50, 57, 55, 53, 48, 51, 56, 53, 56, 57, 54, 54, 52, 56, 56, 57, 56, 49, 53, 53, 52, 52, 51, 53, 56, 48, 48, 55, 53, 56, 34, 44, 34, 116, 105, 109, 101, 115, 116, 97, 109, 112, 34, 58, 34, 50, 48, 50, 51, 45, 48, 56, 45, 50, 54, 84, 48, 50, 58, 48, 56, 58, 52, 51, 46, 53, 51, 52, 57, 53, 55, 34, 44, 34, 118, 101, 114, 115, 105, 111, 110, 34, 58, 52, 44, 34, 101, 112, 105, 100, 80, 115, 101, 117, 100, 111, 110, 121, 109, 34, 58, 34, 55, 43, 74, 112, 105, 53, 82, 83, 68, 117, 97, 54, 113, 49, 114, 109, 90, 72, 77, 112, 75, 100, 116, 80, 104, 102, 103, 111, 81, 106, 43, 55, 52, 85, 106, 111, 49, 49, 83, 67, 113, 118, 89, 107, 47, 85, 49, 47, 115, 48, 56, 72, 50, 109, 67, 101, 107, 110, 53, 119, 107, 82, 47, 53, 114, 71, 55, 66, 49, 122, 114, 104, 74, 49, 107, 121, 118, 101, 82, 47, 107, 72, 120, 98, 47, 71, 109, 75, 71, 51, 84, 99, 52, 83, 86, 74, 110, 78, 55, 75, 43, 47, 97, 48, 80, 110, 54, 121, 43, 106, 66, 75, 104, 106, 52, 79, 74, 101, 109, 102, 88, 75, 84, 104, 109, 49, 104, 89, 122, 52, 76, 65, 113, 53, 120, 69, 83, 82, 121, 68, 76, 89, 111, 71, 82, 89, 47, 57, 48, 118, 103, 97, 65, 89, 90, 109, 104, 53, 111, 85, 111, 48, 65, 98, 98, 111, 49, 99, 65, 102, 119, 61, 34, 44, 34, 97, 100, 118, 105, 115, 111, 114, 121, 85, 82, 76, 34, 58, 34, 104, 116, 116, 112, 115, 58, 47, 47, 115, 101, 99, 117, 114, 105, 116, 121, 45, 99, 101, 110, 116, 101, 114, 46, 105, 110, 116, 101, 108, 46, 99, 111, 109, 34, 44, 34, 97, 100, 118, 105, 115, 111, 114, 121, 73, 68, 115, 34, 58, 91, 34, 73, 78, 84, 69, 76, 45, 83, 65, 45, 48, 48, 51, 51, 52, 34, 44, 34, 73, 78, 84, 69, 76, 45, 83, 65, 45, 48, 48, 51, 56, 49, 34, 44, 34, 73, 78, 84, 69, 76, 45, 83, 65, 45, 48, 48, 51, 56, 57, 34, 44, 34, 73, 78, 84, 69, 76, 45, 83, 65, 45, 48, 48, 52, 55, 55, 34, 44, 34, 73, 78, 84, 69, 76, 45, 83, 65, 45, 48, 48, 54, 49, 52, 34, 44, 34, 73, 78, 84, 69, 76, 45, 83, 65, 45, 48, 48, 54, 49, 53, 34, 44, 34, 73, 78, 84, 69, 76, 45, 83, 65, 45, 48, 48, 54, 49, 55, 34, 44, 34, 73, 78, 84, 69, 76, 45, 83, 65, 45, 48, 48, 56, 50, 56, 34, 93, 44, 34, 105, 115, 118, 69, 110, 99, 108, 97, 118, 101, 81, 117, 111, 116, 101, 83, 116, 97, 116, 117, 115, 34, 58, 34, 71, 82, 79, 85, 80, 95, 79, 85, 84, 95, 79, 70, 95, 68, 65, 84, 69, 34, 44, 34, 112, 108, 97, 116, 102, 111, 114, 109, 73, 110, 102, 111, 66, 108, 111, 98, 34, 58, 34, 49, 53, 48, 50, 48, 48, 54, 53, 48, 52, 48, 48, 48, 49, 48, 48, 48, 48, 48, 70, 48, 70, 48, 50, 48, 50, 48, 49, 56, 48, 48, 69, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 68, 48, 48, 48, 48, 48, 67, 48, 48, 48, 48, 48, 48, 48, 50, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 66, 69, 52, 57, 65, 66, 51, 55, 55, 53, 56, 66, 70, 50, 67, 49, 65, 48, 70, 56, 52, 68, 66, 66, 56, 67, 67, 66, 48, 49, 52, 57, 67, 68, 50, 53, 69, 48, 51, 65, 56, 68, 66, 69, 48, 67, 50, 51, 70, 70, 66, 51, 49, 56, 66, 69, 70, 48, 68, 48, 66, 65, 49, 70, 53, 56, 51, 49, 52, 57, 55, 52, 49, 49, 67, 65, 55, 51, 70, 53, 56, 55, 49, 48, 65, 57, 52, 54, 66, 66, 70, 53, 66, 52, 56, 52, 66, 66, 66, 49, 67, 70, 69, 67, 54, 50, 49, 55, 67, 54, 55, 67, 70, 48, 56, 68, 68, 52, 69, 50, 70, 56, 67, 50, 56, 66, 69, 53, 52, 49, 66, 34, 44, 34, 105, 115, 118, 69, 110, 99, 108, 97, 118, 101, 81, 117, 111, 116, 101, 66, 111, 100, 121, 34, 58, 34, 65, 103, 65, 66, 65, 79, 81, 76, 65, 65, 65, 79, 65, 65, 52, 65, 65, 65, 65, 65, 65, 66, 54, 88, 104, 48, 69, 77, 82, 89, 43, 87, 101, 43, 48, 55, 65, 103, 99, 120, 111, 115, 85, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 67, 103, 55, 47, 66, 119, 101, 65, 66, 103, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 66, 119, 65, 65, 65, 65, 65, 65, 65, 65, 65, 72, 65, 65, 65, 65, 65, 65, 65, 65, 65, 75, 116, 102, 122, 78, 76, 100, 106, 54, 79, 100, 85, 99, 83, 87, 50, 77, 54, 66, 106, 83, 98, 116, 50, 79, 57, 100, 99, 122, 120, 103, 47, 111, 84, 106, 105, 51, 76, 84, 56, 87, 53, 69, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 67, 75, 116, 109, 66, 88, 114, 66, 121, 55, 49, 74, 43, 109, 82, 100, 107, 51, 65, 73, 57, 86, 68, 70, 118, 84, 86, 97, 56, 104, 50, 43, 112, 54, 99, 57, 87, 74, 51, 51, 79, 67, 90, 81, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 68, 68, 114, 56, 121, 89, 72, 57, 57, 79, 89, 52, 111, 68, 106, 111, 57, 80, 79, 111, 51, 103, 83, 47, 56, 75, 43, 114, 48, 56, 99, 86, 66, 110, 77, 107, 110, 88, 121, 89, 117, 43, 109, 119, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 34, 125].to_vec().try_into().unwrap(),
//         sign: [111, 117, 80, 86, 83, 122, 54, 66, 89, 121, 86, 83, 108, 73, 71, 104, 116, 120, 57, 66, 70, 120, 43, 54, 84, 106, 105, 121, 80, 48, 97, 119, 114, 101, 74, 83, 86, 51, 111, 65, 55, 69, 84, 67, 122, 82, 67, 101, 52, 53, 115, 77, 81, 66, 52, 78, 57, 48, 105, 115, 111, 88, 86, 52, 119, 68, 74, 86, 71, 71, 71, 76, 113, 56, 99, 70, 65, 116, 48, 52, 50, 53, 57, 90, 80, 75, 111, 70, 111, 122, 77, 83, 110, 77, 111, 119, 118, 87, 113, 54, 56, 78, 68, 112, 108, 85, 105, 69, 122, 90, 118, 106, 77, 99, 82, 111, 89, 81, 75, 54, 105, 108, 117, 77, 87, 66, 98, 85, 120, 119, 89, 52, 85, 80, 43, 83, 71, 82, 108, 84, 85, 110, 84, 49, 56, 87, 100, 79, 104, 71, 71, 122, 69, 79, 78, 115, 86, 43, 69, 107, 114, 47, 48, 80, 103, 57, 81, 99, 115, 49, 84, 120, 71, 86, 43, 49, 70, 57, 120, 51, 74, 118, 90, 87, 98, 75, 84, 66, 53, 82, 47, 50, 109, 113, 77, 115, 67, 100, 105, 112, 120, 109, 120, 73, 55, 75, 65, 53, 106, 82, 113, 57, 47, 67, 77, 68, 102, 111, 52, 72, 122, 86, 79, 106, 110, 78, 118, 122, 88, 51, 110, 83, 69, 100, 122, 108, 83, 65, 85, 117, 101, 105, 73, 87, 109, 51, 47, 89, 103, 51, 80, 69, 76, 72, 117, 118, 114, 115, 56, 116, 47, 73, 66, 122, 122, 104, 47, 72, 81, 114, 52, 104, 106, 81, 68, 119, 111, 87, 65, 108, 51, 86, 97, 76, 106, 105, 107, 68, 81, 49, 75, 48, 122, 54, 90, 82, 88, 82, 79, 70, 103, 114, 97, 73, 68, 43, 112, 54, 65, 57, 68, 116, 84, 74, 119, 97, 68, 83, 101, 102, 78, 82, 97, 65, 120, 51, 87, 69, 106, 122, 113, 43, 49, 103, 85, 78, 77, 116, 65, 85, 75, 103, 61, 61].to_vec().try_into().unwrap(),
//         cert_der: [77, 73, 73, 69, 111, 84, 67, 67, 65, 119, 109, 103, 65, 119, 73, 66, 65, 103, 73, 74, 65, 78, 69, 72, 100, 108, 48, 121, 111, 55, 67, 87, 77, 65, 48, 71, 67, 83, 113, 71, 83, 73, 98, 51, 68, 81, 69, 66, 67, 119, 85, 65, 77, 72, 52, 120, 67, 122, 65, 74, 66, 103, 78, 86, 66, 65, 89, 84, 65, 108, 86, 84, 77, 81, 115, 119, 67, 81, 89, 68, 86, 81, 81, 73, 68, 65, 74, 68, 81, 84, 69, 85, 77, 66, 73, 71, 65, 49, 85, 69, 66, 119, 119, 76, 85, 50, 70, 117, 100, 71, 69, 103, 81, 50, 120, 104, 99, 109, 69, 120, 71, 106, 65, 89, 66, 103, 78, 86, 66, 65, 111, 77, 69, 85, 108, 117, 100, 71, 86, 115, 73, 69, 78, 118, 99, 110, 66, 118, 99, 109, 70, 48, 97, 87, 57, 117, 77, 84, 65, 119, 76, 103, 89, 68, 86, 81, 81, 68, 68, 67, 100, 74, 98, 110, 82, 108, 98, 67, 66, 84, 82, 49, 103, 103, 81, 88, 82, 48, 90, 88, 78, 48, 89, 88, 82, 112, 98, 50, 52, 103, 85, 109, 86, 119, 98, 51, 74, 48, 73, 70, 78, 112, 90, 50, 53, 112, 98, 109, 99, 103, 81, 48, 69, 119, 72, 104, 99, 78, 77, 84, 89, 120, 77, 84, 73, 121, 77, 68, 107, 122, 78, 106, 85, 52, 87, 104, 99, 78, 77, 106, 89, 120, 77, 84, 73, 119, 77, 68, 107, 122, 78, 106, 85, 52, 87, 106, 66, 55, 77, 81, 115, 119, 67, 81, 89, 68, 86, 81, 81, 71, 69, 119, 74, 86, 85, 122, 69, 76, 77, 65, 107, 71, 65, 49, 85, 69, 67, 65, 119, 67, 81, 48, 69, 120, 70, 68, 65, 83, 66, 103, 78, 86, 66, 65, 99, 77, 67, 49, 78, 104, 98, 110, 82, 104, 73, 69, 78, 115, 89, 88, 74, 104, 77, 82, 111, 119, 71, 65, 89, 68, 86, 81, 81, 75, 68, 66, 70, 74, 98, 110, 82, 108, 98, 67, 66, 68, 98, 51, 74, 119, 98, 51, 74, 104, 100, 71, 108, 118, 98, 106, 69, 116, 77, 67, 115, 71, 65, 49, 85, 69, 65, 119, 119, 107, 83, 87, 53, 48, 90, 87, 119, 103, 85, 48, 100, 89, 73, 69, 70, 48, 100, 71, 86, 122, 100, 71, 70, 48, 97, 87, 57, 117, 73, 70, 74, 108, 99, 71, 57, 121, 100, 67, 66, 84, 97, 87, 100, 117, 97, 87, 53, 110, 77, 73, 73, 66, 73, 106, 65, 78, 66, 103, 107, 113, 104, 107, 105, 71, 57, 119, 48, 66, 65, 81, 69, 70, 65, 65, 79, 67, 65, 81, 56, 65, 77, 73, 73, 66, 67, 103, 75, 67, 65, 81, 69, 65, 113, 88, 111, 116, 52, 79, 90, 117, 112, 104, 82, 56, 110, 117, 100, 70, 114, 65, 70, 105, 97, 71, 120, 120, 107, 103, 109, 97, 47, 69, 115, 47, 66, 65, 43, 116, 98, 101, 67, 84, 85, 82, 49, 48, 54, 65, 76, 49, 69, 78, 99, 87, 65, 52, 70, 88, 51, 75, 43, 69, 57, 66, 66, 76, 48, 47, 55, 88, 53, 114, 106, 53, 110, 73, 103, 88, 47, 82, 47, 49, 117, 98, 104, 107, 75, 87, 119, 57, 103, 102, 113, 80, 71, 51, 75, 101, 65, 116, 73, 100, 99, 118, 47, 117, 84, 79, 49, 121, 88, 118, 53, 48, 118, 113, 97, 80, 118, 69, 49, 67, 82, 67, 104, 118, 122, 100, 83, 47, 90, 69, 66, 113, 81, 53, 111, 86, 118, 76, 84, 80, 90, 51, 86, 69, 105, 99, 81, 106, 108, 121, 116, 75, 103, 78, 57, 99, 76, 110, 120, 98, 119, 116, 117, 118, 76, 85, 75, 55, 101, 121, 82, 80, 102, 74, 87, 47, 107, 115, 100, 100, 79, 122, 80, 56, 86, 66, 66, 110, 105, 111, 108, 89, 110, 82, 67, 68, 50, 106, 114, 77, 82, 90, 56, 110, 66, 77, 50, 90, 87, 89, 119, 110, 88, 110, 119, 89, 101, 79, 65, 72, 86, 43, 87, 57, 116, 79, 104, 65, 73, 109, 119, 82, 119, 75, 70, 47, 57, 53, 121, 65, 115, 86, 119, 100, 50, 49, 114, 121, 72, 77, 74, 66, 99, 71, 72, 55, 48, 113, 76, 97, 103, 90, 55, 84, 116, 121, 116, 43, 43, 113, 79, 47, 54, 43, 75, 65, 88, 74, 117, 75, 119, 90, 113, 106, 82, 108, 69, 116, 83, 69, 122, 56, 103, 90, 81, 101, 70, 102, 86, 89, 103, 99, 119, 83, 102, 111, 57, 54, 111, 83, 77, 65, 122, 86, 114, 55, 86, 48, 76, 54, 72, 83, 68, 76, 82, 110, 112, 98, 54, 120, 120, 109, 98, 80, 100, 113, 78, 111, 108, 52, 116, 81, 73, 68, 65, 81, 65, 66, 111, 52, 71, 107, 77, 73, 71, 104, 77, 66, 56, 71, 65, 49, 85, 100, 73, 119, 81, 89, 77, 66, 97, 65, 70, 72, 104, 68, 101, 51, 97, 109, 102, 114, 122, 81, 114, 51, 53, 67, 78, 43, 115, 49, 102, 68, 117, 72, 65, 86, 69, 56, 77, 65, 52, 71, 65, 49, 85, 100, 68, 119, 69, 66, 47, 119, 81, 69, 65, 119, 73, 71, 119, 68, 65, 77, 66, 103, 78, 86, 72, 82, 77, 66, 65, 102, 56, 69, 65, 106, 65, 65, 77, 71, 65, 71, 65, 49, 85, 100, 72, 119, 82, 90, 77, 70, 99, 119, 86, 97, 66, 84, 111, 70, 71, 71, 84, 50, 104, 48, 100, 72, 65, 54, 76, 121, 57, 48, 99, 110, 86, 122, 100, 71, 86, 107, 99, 50, 86, 121, 100, 109, 108, 106, 90, 88, 77, 117, 97, 87, 53, 48, 90, 87, 119, 117, 89, 50, 57, 116, 76, 50, 78, 118, 98, 110, 82, 108, 98, 110, 81, 118, 81, 49, 74, 77, 76, 49, 78, 72, 87, 67, 57, 66, 100, 72, 82, 108, 99, 51, 82, 104, 100, 71, 108, 118, 98, 108, 74, 108, 99, 71, 57, 121, 100, 70, 78, 112, 90, 50, 53, 112, 98, 109, 100, 68, 81, 83, 53, 106, 99, 109, 119, 119, 68, 81, 89, 74, 75, 111, 90, 73, 104, 118, 99, 78, 65, 81, 69, 76, 66, 81, 65, 68, 103, 103, 71, 66, 65, 71, 99, 73, 116, 104, 116, 99, 75, 57, 73, 86, 82, 122, 52, 114, 82, 113, 43, 90, 75, 69, 43, 55, 107, 53, 48, 47, 79, 120, 85, 115, 109, 87, 56, 97, 97, 118, 79, 122, 75, 98, 48, 105, 67, 120, 48, 55, 89, 81, 57, 114, 122, 105, 53, 110, 85, 55, 51, 116, 77, 69, 50, 121, 71, 82, 76, 122, 104, 83, 86, 105, 70, 115, 47, 76, 112, 70, 97, 57, 108, 112, 81, 76, 54, 74, 76, 49, 97, 81, 119, 109, 68, 82, 55, 52, 84, 120, 89, 71, 66, 65, 73, 105, 53, 102, 52, 73, 53, 84, 74, 111, 67, 67, 69, 113, 82, 72, 122, 57, 49, 107, 112, 71, 54, 85, 118, 121, 110, 50, 116, 76, 109, 110, 73, 100, 74, 98, 80, 69, 52, 118, 89, 118, 87, 76, 114, 116, 88, 88, 102, 70, 66, 83, 83, 80, 68, 52, 65, 102, 110, 55, 43, 51, 47, 88, 85, 103, 103, 65, 108, 99, 55, 111, 67, 84, 105, 122, 79, 102, 98, 98, 116, 79, 70, 108, 89, 65, 52, 103, 53, 75, 99, 89, 103, 83, 49, 74, 50, 90, 65, 101, 77, 81, 113, 98, 85, 100, 90, 115, 101, 90, 67, 99, 97, 90, 90, 90, 110, 54, 53, 116, 100, 113, 101, 101, 56, 85, 88, 90, 108, 68, 118, 120, 48, 43, 78, 100, 79, 48, 76, 82, 43, 53, 112, 70, 121, 43, 106, 117, 77, 48, 119, 87, 98, 117, 53, 57, 77, 118, 122, 99, 109, 84, 88, 98, 106, 115, 105, 55, 72, 89, 54, 122, 100, 53, 51, 89, 113, 53, 75, 50, 52, 52, 102, 119, 70, 72, 82, 81, 56, 101, 79, 66, 48, 73, 87, 66, 43, 52, 80, 102, 77, 55, 70, 101, 65, 65, 112, 90, 118, 108, 102, 113, 108, 75, 79, 108, 76, 99, 90, 76, 50, 117, 121, 86, 109, 122, 82, 107, 121, 82, 53, 121, 87, 55, 50, 117, 111, 57, 109, 101, 104, 88, 52, 52, 67, 105, 80, 74, 50, 102, 115, 101, 57, 89, 54, 101, 81, 116, 99, 102, 69, 104, 77, 80, 107, 109, 72, 88, 73, 48, 49, 115, 78, 43, 75, 119, 80, 98, 112, 65, 51, 57, 43, 120, 79, 115, 83, 116, 106, 104, 80, 57, 78, 49, 89, 49, 97, 50, 116, 81, 65, 86, 111, 43, 121, 86, 103, 76, 103, 86, 50, 72, 119, 115, 55, 51, 70, 99, 48, 111, 51, 119, 67, 55, 56, 113, 80, 69, 65, 43, 118, 50, 97, 82, 115, 47, 66, 101, 51, 90, 70, 68, 103, 68, 121, 103, 104, 99, 47, 49, 102, 103, 85, 43, 55, 67, 43, 80, 54, 107, 98, 113, 100, 52, 112, 111, 121, 98, 54, 73, 87, 56, 75, 67, 74, 98, 120, 102, 77, 74, 118, 107, 111, 114, 100, 78, 79, 103, 79, 85, 85, 120, 110, 100, 80, 72, 69, 105, 47, 116, 98, 47, 85, 55, 117, 76, 106, 76, 79, 103, 80, 65, 61, 61].to_vec().try_into().unwrap(),
//     }
// }

// pub fn tee_register<T: Config>() -> Result<(T::AccountId, T::AccountId), &'static str> {
//     let (stash, controller) = pallet_cess_staking::testing_utils::create_stash_controller::<T>(USER_SEED, 100, Default::default())?;
//     let sgx_att_report = get_report();
//     TeeWorker::<T>::register(
//         RawOrigin::Signed(controller.clone()).into(),
//         stash.clone(),
//         NODE_PUBLIC_KEY,
//         PEER_ID, 
//         PODR2_PBK, 
//         sgx_att_report
//     ).map_err(|_| "tee worker register error")?;
//     Ok((stash, controller))
// }

// benchmarks! {
//     register {
//         let (stash, controller) = pallet_cess_staking::testing_utils::create_stash_controller::<T>(USER_SEED, 100, Default::default())?;
//         let sgx_att_report = get_report();
//     }: _(RawOrigin::Signed(controller.clone()), stash.clone(), NODE_PUBLIC_KEY, PEER_ID, PODR2_PBK, sgx_att_report)
//     verify {
//         assert!(TeeWorkerMap::<T>::contains_key(&controller))
//     }

//     update_whitelist {
//         let _ = tee_register::<T>()?;
//         let mr_enclave = [5u8; 64];
//     }: _(RawOrigin::Root, mr_enclave.clone())
//     verify {
//         let mr_enclave_list = <MrEnclaveWhitelist<T>>::get();
//         assert!(mr_enclave_list.contains(&mr_enclave))
//     }

//     exit {
//         let (_, controller) = tee_register::<T>()?;
//     }: _(RawOrigin::Signed(controller.clone()))
//     verify {
//         assert!(!TeeWorkerMap::<T>::contains_key(&controller))
//     }
// }

