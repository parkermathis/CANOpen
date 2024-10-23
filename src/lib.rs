#[allow(non_snake_case)]
use canlib_rs::*;

pub enum NmtServices {
    StartRemoteNode = 1,
    StopRemoteNode = 2,
    EnterPreOperational = 128,
    ResetNode = 129,
    ResetCommunication = 130,
}

pub enum NodeGuardingState {
    Initialising = 0,
    Disconnected = 1,
    Connecting = 2,
    Preparing = 3,
    Stopped = 4,
    Operational = 5,
    PreOperational = 127,
}

pub fn nmt_module_control (hnd: CanHandle, node_id: u8, cs: NmtServices) -> CanStatus {
    let mut data: [u8;2] = [cs as u8, node_id];
    
    can_write(hnd, 0x000, &mut data, 2, CanMsg::Std as u32)
}
    
pub fn nmt_node_guarding (hnd: CanHandle, node_id: u8) -> (CanStatus, u8, u8) {
    let mut status;
    let cob_id: u16 = 0x700 + node_id;

    //TODO: Timeout should be argument
    status = can_write_wait(hnd, cob_id as i64, &mut [], 0, CanMsg::Rtr as u32, 250);
    if status != CanStatus::CanOk { 
        return (status, 0, 0);
    }

    status = can_read_sync_specific(hnd, cob_id, 250);
    if status != CanStatus::CanOk {
        return (status, 0, 0);
    }

    let mut rx_msg: [u8;8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut rx_dlc: u32 = 0;
    let mut rx_flag: u32 = 0;
    let mut rx_time: u32 = 0;

    status = can_read_specific_skip(hnd, cob_id, &mut rx_msg, &mut rx_dlc, &mut rx_flag, &mut rx_time);
    if status != CanStatus::CanOk {
        return (status, 0, 0);
    }

    if rx_dlc != 1 {
        return (status, 255, 0);
    }

    let state = rx_msg[0] & 127;
    let toggle = rx_msg[0] >> 7;

    (status, state, toggle)
}

pub fn initiate_domain_download (hnd: CanHandle, index: u16, subindex: u8, data: &mut u32, len: u8, node: &u64) -> CanStatus {
    let mut status;

    //TODO: Am I doing this right?
    let mut cs: u8 = 0b00100000;
    cs &= 0b11;
    cs &= (8 - len) as u8;

    let mut msg: [u8;8] = [cs, 
                           (index << 8),
                           (index & 0xFF),
                           subindex,
                           0, 0, 0, 0];

    for i in 0..len { 
        msg[(i as usize) + 4] = ((data >> (8 * i)) as u8) & 0xFF;
    }

    status = can_write(hnd, node as i64, &mut msg, 8, CanMsg::std)
}
