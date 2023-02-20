//! A tx to initialize a new established address with a given public key and
//! a validity predicate.

use namada_tx_prelude::*;

#[transaction]
fn apply_tx(ctx: &mut Ctx, tx_data: Vec<u8>) -> TxResult {
    let signed = SignedTxData::try_from_slice(&tx_data[..])
        .wrap_err("failed to decode SignedTxData")?;
    let data = signed.data.ok_or_err_msg("Missing data")?;
    let tx_data = transaction::pgf::PgfProjectsUpdate::try_from_slice(&data[..])
        .wrap_err("failed to decode UpatePgfProjects")?;
    debug_log!("apply_tx called to update pgf projects");
    let pgf_active_counsil = pgf::get_current_counsil_address(ctx)?;
    let counsil_address = match pgf_active_counsil {
        Some(address) => address,
        _ => return Ok(())
    };
    ctx.insert_verifier(&counsil_address);
    pgf::update_projects(ctx, tx_data)
}
