use crate::{
    ethercatmain::{ecx_context, ecx_contextt, ecx_poperror},
    ethercattype::{ec_err_type, ec_errort, C2RustUnnamed_0},
    osal::linux::osal::ec_timet,
};
use libc::sprintf;

#[derive(Copy, Clone)]
struct ec_sdoerrorlist_t {
    pub errorcode: u32,
    pub errordescription: &'static str,
}

#[derive(Copy, Clone)]
pub struct ec_ALstatuscodelist_t {
    pub ALstatuscode: u16,
    pub ALstatuscodedescription: &'static str,
}

#[derive(Copy, Clone)]
pub struct ec_soeerrorlist_t {
    pub errorcode: u16,
    pub errordescription: &'static str,
}

#[derive(Copy, Clone)]
pub struct ec_mbxerrorlist_t {
    pub errorcode: u16,
    pub errordescription: &'static str,
}
#[no_mangle]
pub static mut estring: [libc::c_char; 127] = [0; 127];
/* * SDO error list definition */
#[no_mangle]
static mut ec_sdoerrorlist: [ec_sdoerrorlist_t; 32] = {
    [
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0u32,
                errordescription: "No error",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x5030000u32,
                errordescription: "Toggle bit not changed",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x5040000u32,
                errordescription: "SDO protocol timeout",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x5040001u32,
                errordescription: "Client/Server command specifier not valid or unknown",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x5040005u32,
                errordescription: "Out of memory",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6010000u32,
                errordescription: "Unsupported access to an object",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6010001u32,
                errordescription: "Attempt to read to a write only object",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6010002u32,
                errordescription: "Attempt to write to a read only object",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6010003u32,
                errordescription: "Subindex can not be written, SI0 must be 0 for write access",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6010004u32,
                errordescription: "SDO Complete access not supported for variable length objects",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6010005u32,
                errordescription: "Object length exceeds mailbox size",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6010006u32,
                errordescription: "Object mapped to RxPDO, SDO download blocked",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6020000u32,
                errordescription: "The object does not exist in the object directory",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6040041u32,
                errordescription: "The object can not be mapped into the PDO",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6040042u32,
                errordescription:
                    "The number and length of the objects to be mapped would exceed the PDO length",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6040043u32,
                errordescription: "General parameter incompatibility reason",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6040047u32,
                errordescription: "General internal incompatibility in the device",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6060000u32,
                errordescription: "Access failed due to a hardware error",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6070010u32,
                errordescription:
                    "Data type does not match, length of service parameter does not match",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6070012u32,
                errordescription: "Data type does not match, length of service parameter too high",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6070013u32,
                errordescription: "Data type does not match, length of service parameter too low",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6090011u32,
                errordescription: "Subindex does not exist",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6090030u32,
                errordescription: "Value range of parameter exceeded (only for write access)",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6090031u32,
                errordescription: "Value of parameter written too high",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6090032u32,
                errordescription: "Value of parameter written too low",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x6090036u32,
                errordescription: "Maximum value is less than minimum value",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x8000000u32,
                errordescription: "General error",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x8000020u32,
                errordescription: "Data cannot be transferred or stored to the application",
            };
            init
        },
        {
            let init =
                 ec_sdoerrorlist_t{errorcode:
                                       0x8000021u32,
                                   errordescription: "Data cannot be transferred or stored to the application because of local control",};
            init
        },
        {
            let init =
                 ec_sdoerrorlist_t{errorcode:
                                       0x8000022u32,
                                   errordescription: "Data cannot be transferred or stored to the application because of the present device state",};
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0x8000023u32,
                errordescription:
                    "Object dictionary dynamic generation fails or no object dictionary is present",
            };
            init
        },
        {
            let init = ec_sdoerrorlist_t {
                errorcode: 0xffffffffu32,
                errordescription: "Unknown",
            };
            init
        },
    ]
};
/* * AL status code list definition */
#[no_mangle]
pub static mut ec_ALstatuscodelist: [ec_ALstatuscodelist_t; 53] = {
    [
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0u16,
                ALstatuscodedescription: "No error",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x1u16,
                ALstatuscodedescription: "Unspecified error",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x2u16,
                ALstatuscodedescription: "No memory",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x11u16,
                ALstatuscodedescription: "Invalid requested state change",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x12u16,
                ALstatuscodedescription: "Unknown requested state",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x13u16,
                ALstatuscodedescription: "Bootstrap not supported",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x14u16,
                ALstatuscodedescription: "No valid firmware",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x15u16,
                ALstatuscodedescription: "Invalid mailbox configuration",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x16u16,
                ALstatuscodedescription: "Invalid mailbox configuration",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x17u16,
                ALstatuscodedescription: "Invalid sync manager configuration",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x18u16,
                ALstatuscodedescription: "No valid inputs available",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x19u16,
                ALstatuscodedescription: "No valid outputs",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x1au16,
                ALstatuscodedescription: "Synchronization error",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x1bu16,
                ALstatuscodedescription: "Sync manager watchdog",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x1cu16,
                ALstatuscodedescription: "Invalid sync Manager types",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x1du16,
                ALstatuscodedescription: "Invalid output configuration",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x1eu16,
                ALstatuscodedescription: "Invalid input configuration",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x1fu16,
                ALstatuscodedescription: "Invalid watchdog configuration",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x20u16,
                ALstatuscodedescription: "Slave needs cold start",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x21u16,
                ALstatuscodedescription: "Slave needs INIT",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x22u16,
                ALstatuscodedescription: "Slave needs PREOP",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x23u16,
                ALstatuscodedescription: "Slave needs SAFEOP",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x24u16,
                ALstatuscodedescription: "Invalid input mapping",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x25u16,
                ALstatuscodedescription: "Invalid output mapping",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x26u16,
                ALstatuscodedescription: "Inconsistent settings",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x27u16,
                ALstatuscodedescription: "Freerun not supported",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x28u16,
                ALstatuscodedescription: "Synchronisation not supported",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x29u16,
                ALstatuscodedescription: "Freerun needs 3buffer mode",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x2au16,
                ALstatuscodedescription: "Background watchdog",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x2bu16,
                ALstatuscodedescription: "No valid Inputs and Outputs",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x2cu16,
                ALstatuscodedescription: "Fatal sync error",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x2du16,
                ALstatuscodedescription: "No sync error",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x2eu16,
                ALstatuscodedescription: "Invalid input FMMU configuration",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x30u16,
                ALstatuscodedescription: "Invalid DC SYNC configuration",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x31u16,
                ALstatuscodedescription: "Invalid DC latch configuration",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x32u16,
                ALstatuscodedescription: "PLL error",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x33u16,
                ALstatuscodedescription: "DC sync IO error",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x34u16,
                ALstatuscodedescription: "DC sync timeout error",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x35u16,
                ALstatuscodedescription: "DC invalid sync cycle time",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x36u16,
                ALstatuscodedescription: "DC invalid sync0 cycle time",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x37u16,
                ALstatuscodedescription: "DC invalid sync1 cycle time",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x41u16,
                ALstatuscodedescription: "MBX_AOE",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x42u16,
                ALstatuscodedescription: "MBX_EOE",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x43u16,
                ALstatuscodedescription: "MBX_COE",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x44u16,
                ALstatuscodedescription: "MBX_FOE",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x45u16,
                ALstatuscodedescription: "MBX_SOE",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x4fu16,
                ALstatuscodedescription: "MBX_VOE",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x50u16,
                ALstatuscodedescription: "EEPROM no access",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x51u16,
                ALstatuscodedescription: "EEPROM error",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x60u16,
                ALstatuscodedescription: "Slave restarted locally",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0x61u16,
                ALstatuscodedescription: "Device identification value updated",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0xf0u16,
                ALstatuscodedescription: "Application controller available",
            };
            init
        },
        {
            let init = ec_ALstatuscodelist_t {
                ALstatuscode: 0xffffu16,
                ALstatuscodedescription: "Unknown",
            };
            init
        },
    ]
};
/* * SoE error list definition */
#[no_mangle]
pub static mut ec_soeerrorlist: [ec_soeerrorlist_t; 51] = {
    [
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0u16,
                errordescription: "No error",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x1001u16,
                errordescription: "No IDN",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x1009u16,
                errordescription: "Invalid access to element 1",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x2001u16,
                errordescription: "No Name",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x2002u16,
                errordescription: "Name transmission too short",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x2003u16,
                errordescription: "Name transmission too long",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x2004u16,
                errordescription: "Name cannot be changed (read only)",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x2005u16,
                errordescription: "Name is write-protected at this time",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x3002u16,
                errordescription: "Attribute transmission too short",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x3003u16,
                errordescription: "Attribute transmission too long",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x3004u16,
                errordescription: "Attribute cannot be changed (read only)",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x3005u16,
                errordescription: "Attribute is write-protected at this time",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x4001u16,
                errordescription: "No units",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x4002u16,
                errordescription: "Unit transmission too short",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x4003u16,
                errordescription: "Unit transmission too long",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x4004u16,
                errordescription: "Unit cannot be changed (read only)",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x4005u16,
                errordescription: "Unit is write-protected at this time",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x5001u16,
                errordescription: "No minimum input value",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x5002u16,
                errordescription: "Minimum input value transmission too short",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x5003u16,
                errordescription: "Minimum input value transmission too long",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x5004u16,
                errordescription: "Minimum input value cannot be changed (read only)",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x5005u16,
                errordescription: "Minimum input value is write-protected at this time",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x6001u16,
                errordescription: "No maximum input value",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x6002u16,
                errordescription: "Maximum input value transmission too short",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x6003u16,
                errordescription: "Maximum input value transmission too long",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x6004u16,
                errordescription: "Maximum input value cannot be changed (read only)",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x6005u16,
                errordescription: "Maximum input value is write-protected at this time",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x7002u16,
                errordescription: "Operation data transmission too short",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x7003u16,
                errordescription: "Operation data transmission too long",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x7004u16,
                errordescription: "Operation data cannot be changed (read only)",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x7005u16,
                errordescription: "Operation data is write-protected at this time (state)",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x7006u16,
                errordescription: "Operation data is smaller than the minimum input value",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x7007u16,
                errordescription: "Operation data is smaller than the maximum input value",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x7008u16,
                errordescription: "Invalid operation data:Configured IDN will not be supported",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x7009u16,
                errordescription: "Operation data write protected by a password",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x700au16,
                errordescription: "Operation data is write protected, it is configured cyclically",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x700bu16,
                errordescription:
                    "Invalid indirect addressing: (e.g., data container, list handling)",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x700cu16,
                errordescription: "Operation data is write protected, due to other settings",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x700du16,
                errordescription: "Reserved",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x7010u16,
                errordescription: "Procedure command already active",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x7011u16,
                errordescription: "Procedure command not interruptible",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x7012u16,
                errordescription: "Procedure command at this time not executable (state)",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x7013u16,
                errordescription: "Procedure command not executable (invalid or false parameters)",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x7014u16,
                errordescription: "No data state",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x8001u16,
                errordescription: "No default value",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x8002u16,
                errordescription: "Default value transmission too long",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x8004u16,
                errordescription: "Default value cannot be changed, read only",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x800au16,
                errordescription: "Invalid drive number",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x800au16,
                errordescription: "General error",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0x800au16,
                errordescription: "No element addressed",
            };
            init
        },
        {
            let init = ec_soeerrorlist_t {
                errorcode: 0xffffu16,
                errordescription: "Unknown",
            };
            init
        },
    ]
};
/* * MBX error list definition */
#[no_mangle]
pub static mut ec_mbxerrorlist: [ec_mbxerrorlist_t; 10] = {
    [
        {
            let init = ec_mbxerrorlist_t {
                errorcode: 0u16,
                errordescription: "No error",
            };
            init
        },
        {
            let init = ec_mbxerrorlist_t {
                errorcode: 0x1u16,
                errordescription: "Syntax of 6 octet Mailbox Header is wrong",
            };
            init
        },
        {
            let init = ec_mbxerrorlist_t {
                errorcode: 0x2u16,
                errordescription: "The mailbox protocol is not supported",
            };
            init
        },
        {
            let init = ec_mbxerrorlist_t {
                errorcode: 0x3u16,
                errordescription: "Channel Field contains wrong value",
            };
            init
        },
        {
            let init = ec_mbxerrorlist_t {
                errorcode: 0x4u16,
                errordescription: "The service is no supported",
            };
            init
        },
        {
            let init = ec_mbxerrorlist_t {
                errorcode: 0x5u16,
                errordescription: "Invalid mailbox header",
            };
            init
        },
        {
            let init = ec_mbxerrorlist_t {
                errorcode: 0x6u16,
                errordescription: "Length of received mailbox data is too short",
            };
            init
        },
        {
            let init = ec_mbxerrorlist_t {
                errorcode: 0x7u16,
                errordescription: "No more memory in slave",
            };
            init
        },
        {
            let init = ec_mbxerrorlist_t {
                errorcode: 0x8u16,
                errordescription: "The length of data is inconsistent",
            };
            init
        },
        {
            let init = ec_mbxerrorlist_t {
                errorcode: 0xffffu16,
                errordescription: "Unknown",
            };
            init
        },
    ]
};
/* * Look up text string that belongs to SDO error code.
 *
 * @param[in] sdoerrorcode   = SDO error code as defined in EtherCAT protocol
 * @return readable string
 */
#[no_mangle]
pub unsafe fn ec_sdoerror2string(sdoerrorcode: u32) -> &'static str {
    let mut i: libc::c_int = 0i32;
    while ec_sdoerrorlist[i as usize].errorcode as libc::c_ulong != 0xffffffffu64
        && ec_sdoerrorlist[i as usize].errorcode != sdoerrorcode
    {
        i += 1
    }
    return ec_sdoerrorlist[i as usize].errordescription;
}
/* * Look up text string that belongs to AL status code.
 *
 * @param[in] ALstatuscode   = AL status code as defined in EtherCAT protocol
 * @return readable string
 */
#[no_mangle]
pub unsafe fn ec_ALstatuscode2string(ALstatuscode: u16) -> *mut libc::c_char {
    let mut i: libc::c_int = 0i32;
    while ec_ALstatuscodelist[i as usize].ALstatuscode as libc::c_int != 0xffffi32
        && ec_ALstatuscodelist[i as usize].ALstatuscode as libc::c_int
            != ALstatuscode as libc::c_int
    {
        i += 1
    }
    return ec_ALstatuscodelist[i as usize]
        .ALstatuscodedescription
        .as_ptr() as *mut libc::c_char;
}
/* * Look up text string that belongs to SoE error code.
 *
 * @param[in] errorcode   = SoE error code as defined in EtherCAT protocol
 * @return readable string
 */
#[no_mangle]
pub unsafe fn ec_soeerror2string(errorcode: u16) -> *mut libc::c_char {
    let mut i: libc::c_int = 0i32;
    while ec_soeerrorlist[i as usize].errorcode as libc::c_int != 0xffffi32
        && ec_soeerrorlist[i as usize].errorcode as libc::c_int != errorcode as libc::c_int
    {
        i += 1
    }
    return ec_soeerrorlist[i as usize].errordescription.as_ptr() as *mut libc::c_char;
}
/* * Look up text string that belongs to MBX error code.
 *
 * @param[in] errorcode   = MBX error code as defined in EtherCAT protocol
 * @return readable string
 */
#[no_mangle]
pub unsafe fn ec_mbxerror2string(errorcode: u16) -> *mut libc::c_char {
    let mut i: libc::c_int = 0i32;
    while ec_mbxerrorlist[i as usize].errorcode as libc::c_int != 0xffffi32
        && ec_mbxerrorlist[i as usize].errorcode as libc::c_int != errorcode as libc::c_int
    {
        i += 1
    }
    return ec_mbxerrorlist[i as usize].errordescription.as_ptr() as *mut libc::c_char;
}
/* * Convert an error to text string.
 *
 * @param[in] Ec = Struct describing the error.
 * @return readable string
 */
#[no_mangle]
pub unsafe fn ecx_err2string(Ec: ec_errort) -> *mut libc::c_char {
    let mut timestr: [libc::c_char; 20] = [0; 20];
    sprintf(
        timestr.as_mut_ptr(),
        b"Time:%12.3f" as *const u8 as *const libc::c_char,
        Ec.Time.sec as libc::c_double + Ec.Time.usec as libc::c_double / 1000000.0f64,
    );
    match Ec.Etype {
        ec_err_type::EC_ERR_TYPE_SDO_ERROR => {
            sprintf(
                estring.as_mut_ptr(),
                b"%s SDO slave:%d index:%4.4x.%2.2x error:%8.8x %s\n" as *const u8
                    as *const libc::c_char,
                timestr.as_mut_ptr(),
                Ec.Slave as libc::c_int,
                Ec.Index as libc::c_int,
                Ec.SubIdx as libc::c_int,
                Ec.c2rust_unnamed.AbortCode as libc::c_uint,
                ec_sdoerror2string(Ec.c2rust_unnamed.AbortCode as u32),
            );
        }
        ec_err_type::EC_ERR_TYPE_EMERGENCY => {
            sprintf(
                estring.as_mut_ptr(),
                b"%s EMERGENCY slave:%d error:%4.4x\n" as *const u8 as *const libc::c_char,
                timestr.as_mut_ptr(),
                Ec.Slave as libc::c_int,
                Ec.c2rust_unnamed.c2rust_unnamed.ErrorCode as libc::c_int,
            );
        }
        ec_err_type::EC_ERR_TYPE_PACKET_ERROR => {
            sprintf(
                estring.as_mut_ptr(),
                b"%s PACKET slave:%d index:%4.4x.%2.2x error:%d\n" as *const u8
                    as *const libc::c_char,
                timestr.as_mut_ptr(),
                Ec.Slave as libc::c_int,
                Ec.Index as libc::c_int,
                Ec.SubIdx as libc::c_int,
                Ec.c2rust_unnamed.c2rust_unnamed.ErrorCode as libc::c_int,
            );
        }
        ec_err_type::EC_ERR_TYPE_SDOINFO_ERROR => {
            sprintf(
                estring.as_mut_ptr(),
                b"%s SDO slave:%d index:%4.4x.%2.2x error:%8.8x %s\n" as *const u8
                    as *const libc::c_char,
                timestr.as_mut_ptr(),
                Ec.Slave as libc::c_int,
                Ec.Index as libc::c_int,
                Ec.SubIdx as libc::c_int,
                Ec.c2rust_unnamed.AbortCode as libc::c_uint,
                ec_sdoerror2string(Ec.c2rust_unnamed.AbortCode as u32),
            );
        }
        ec_err_type::EC_ERR_TYPE_SOE_ERROR => {
            sprintf(
                estring.as_mut_ptr(),
                b"%s SoE slave:%d IDN:%4.4x error:%4.4x %s\n" as *const u8 as *const libc::c_char,
                timestr.as_mut_ptr(),
                Ec.Slave as libc::c_int,
                Ec.Index as libc::c_int,
                Ec.c2rust_unnamed.AbortCode as libc::c_uint,
                ec_soeerror2string(Ec.c2rust_unnamed.c2rust_unnamed.ErrorCode),
            );
        }
        ec_err_type::EC_ERR_TYPE_MBX_ERROR => {
            sprintf(
                estring.as_mut_ptr(),
                b"%s MBX slave:%d error:%4.4x %s\n" as *const u8 as *const libc::c_char,
                timestr.as_mut_ptr(),
                Ec.Slave as libc::c_int,
                Ec.c2rust_unnamed.c2rust_unnamed.ErrorCode as libc::c_int,
                ec_mbxerror2string(Ec.c2rust_unnamed.c2rust_unnamed.ErrorCode),
            );
        }
        _ => {
            sprintf(
                estring.as_mut_ptr(),
                b"%s error:%8.8x\n" as *const u8 as *const libc::c_char,
                timestr.as_mut_ptr(),
                Ec.c2rust_unnamed.AbortCode as libc::c_uint,
            );
        }
    }
    return estring.as_mut_ptr();
}
/* * Look up error in ec_errorlist and convert to text string.
 *
 * @param[in]  context        = context struct
 * @return readable string
 */
#[no_mangle]
pub unsafe fn ecx_elist2string(context: *mut ecx_contextt) -> *mut libc::c_char {
    let mut Ec: ec_errort = ec_errort {
        Time: ec_timet { sec: 0, usec: 0 },
        Signal: false,
        Slave: 0,
        Index: 0,
        SubIdx: 0,
        Etype: ec_err_type::EC_ERR_TYPE_SDO_ERROR,
        c2rust_unnamed: C2RustUnnamed_0 { AbortCode: 0 },
    };
    if ecx_poperror(context, &mut Ec) != false {
        return ecx_err2string(Ec);
    } else {
        return b"" as *const u8 as *mut libc::c_char;
    };
}
#[no_mangle]
pub unsafe fn ec_elist2string() -> *mut libc::c_char {
    return ecx_elist2string(&mut ecx_context);
}
