use rand::RngCore;
use zcash_client_backend::encoding::encode_payment_address;
use zcash_primitives::consensus::MainNetwork;
use zcash_primitives::zip32::ExtendedSpendingKey;

/// Resultado de geração de chaves Zcash.
pub struct ZcashKeys {
    pub spending_key: String,
    pub viewing_key: String,
    pub address: String,
    pub seed: String,
}

/// Gera chaves Zcash Sapling.
pub fn generate_keys() -> anyhow::Result<ZcashKeys> {
    let mut seed = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);
    let extsk = ExtendedSpendingKey::master(&seed);
    let exfvk = extsk.to_extended_full_viewing_key();
    let (address, _) = exfvk.default_address();
    let encoded = encode_payment_address(MainNetwork, &address);
    Ok(ZcashKeys {
        spending_key: hex::encode(extsk.to_bytes()),
        viewing_key: hex::encode(exfvk.to_bytes()),
        address: encoded,
        seed: hex::encode(seed),
    })
}
