//! XX (Noise Protocol) implementation of an Ockam Key Exchanger.
//!
//! This crate contains the key exchange types of the Ockam library and is intended
//! for use by other crates that provide features and add-ons to the main
//! Ockam library.
//!
//! The main Ockam crate re-exports types defined in this crate.
#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    warnings
)]

mod error;
pub use error::*;

/// The number of bytes in a SHA256 digest
pub const SHA256_SIZE: usize = 32;
/// The number of bytes in AES-GCM tag
pub const AES_GCM_TAGSIZE: usize = 16;

mod initiator;
mod state;
pub use initiator::*;
mod responder;
pub use responder::*;
mod new_key_exchanger;
pub use new_key_exchanger::*;

#[cfg(test)]
mod tests {
    use super::*;
    use ockam_key_exchange_core::{KeyExchanger, NewKeyExchanger};
    use ockam_vault::SoftwareVault;
    use ockam_vault_core::SecretVault;
    use ockam_vault_sync_core::Vault;

    #[allow(non_snake_case)]
    #[test]
    fn full_flow__correct_credentials__keys_should_match() {
        let (mut ctx, mut executor) = ockam_node::start_node();
        executor
            .execute(async move {
                let mut vault = Vault::start(&ctx, SoftwareVault::default()).await.unwrap();

                let key_exchanger = XXNewKeyExchanger::new(vault.start_another().unwrap());

                let mut initiator = key_exchanger.initiator().unwrap();
                let mut responder = key_exchanger.responder().unwrap();

                let m1 = initiator.process(&[]).unwrap();
                let _ = responder.process(&m1).unwrap();
                let m2 = responder.process(&[]).unwrap();
                let _ = initiator.process(&m2).unwrap();
                let m3 = initiator.process(&[]).unwrap();
                let _ = responder.process(&m3).unwrap();

                let initiator = Box::new(initiator);
                let initiator = initiator.finalize().unwrap();
                let responder = Box::new(responder);
                let responder = responder.finalize().unwrap();

                assert_eq!(initiator.h(), responder.h());

                let s1 = vault.secret_export(&initiator.encrypt_key()).unwrap();
                let s2 = vault.secret_export(&responder.decrypt_key()).unwrap();

                assert_eq!(s1, s2);

                let s1 = vault.secret_export(&initiator.decrypt_key()).unwrap();
                let s2 = vault.secret_export(&responder.encrypt_key()).unwrap();

                assert_eq!(s1, s2);

                ctx.stop().await.unwrap()
            })
            .unwrap();
    }
}
