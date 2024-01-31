use crate::parsing::types::DeployContract;
use crate::sign_ui;
use crate::{
    parsing::{borsh::BorshDeserialize, HashingStream, SingleTxStream},
    AppSW,
};

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    ordinal_action: u32,
    total_actions: u32,
) -> Result<(), AppSW> {
    let deploy_contract =
        DeployContract::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    #[cfg(feature = "speculos")]
    deploy_contract.debug_print();

    if !sign_ui::action::ui_display_deploy_contract(
        &deploy_contract,
        ordinal_action + 1,
        total_actions,
    ) {
        return Err(AppSW::Deny);
    }
    Ok(())
}