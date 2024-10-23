#[allow(non_snake_case)]
use canlib_rs::*;

pub enum NmtServices {
    StartRemoteNode = 1,
    StopRemoteNode = 2,
    EnterPreOperational = 128,
    ResetNode = 129,
    ResetCommunication = 130,
}

pub fn nmt_module_control (hnd: CanHandle, node_id: u8, cs: NmtServices) -> CanStatus {
    let mut data: [u8;2] = [cs as u8, node_id];
    
    can_write(hnd, 0x000, &mut data, 2, CanMsg::Std as u32)
}
    
pub fn nmt_node_guarding (hnd: CanHandle, node_id: u8) -> (CanStatus, u8, u8) {
    let status;
    let cob_id: u16 = 0x700 + node_id;

    sta
