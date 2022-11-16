use strum_macros::{EnumIter, EnumString};

pub struct InputRegisterValue {
    pub address: u16,
    pub name: &'static str,
    pub unit_symbol: &'static str,
    #[allow(unused)]
    pub unit_name: &'static str,
}

#[derive(EnumString, EnumIter)]
pub enum InputRegister {
    Phase1Line2NeutralVoltage,
    Phase2Line2NeutralVoltage,
    Phase3Line2NeutralVoltage,
    Phase1Current,
    Phase2Current,
    Phase3Current,
    Phase1Power,
    Phase2Power,
    Phase3Power,
    Phase1ApparentPower,
    Phase2ApparentPower,
    Phase3ApparentPower,
    Phase1ReactivePower,
    Phase2ReactivePower,
    Phase3ReactivePower,
    Phase1PowerFactor,
    Phase2PowerFactor,
    Phase3PowerFactor,
    Phase1PhaseAngle,
    Phase2PhaseAngle,
    Phase3PhaseAngle,
    AvgLineToNeutralVoltage,
    AvgLineToNeutralCurrent,
    SumOfLineCurrents,
    TotalSystemPower,
    FrequencyVoltageLines,
}

pub fn generate_input_register(x: InputRegister) -> Box<InputRegisterValue> {
    match x {
        InputRegister::Phase1Line2NeutralVoltage => Box::new(InputRegisterValue {
            address: 0x0000,
            name: "Phase 1 line to neutral voltage",
            unit_symbol: "V",
            unit_name: "Volts",
        }),
        InputRegister::Phase2Line2NeutralVoltage => Box::new(InputRegisterValue {
            address: 0x0002,
            name: "Phase 2 line to neutral voltage",
            unit_symbol: "V",
            unit_name: "Volts",
        }),
        InputRegister::Phase3Line2NeutralVoltage => Box::new(InputRegisterValue {
            address: 0x0004,
            name: "Phase 3 line to neutral voltage",
            unit_symbol: "V",
            unit_name: "Volts",
        }),
        InputRegister::Phase1Current => Box::new(InputRegisterValue {
            address: 0x0006,
            name: "Phase 1 current",
            unit_symbol: "A",
            unit_name: "Amps",
        }),
        InputRegister::Phase2Current => Box::new(InputRegisterValue {
            address: 0x0008,
            name: "Phase 2 current",
            unit_symbol: "A",
            unit_name: "Amps",
        }),
        InputRegister::Phase3Current => Box::new(InputRegisterValue {
            address: 0x000A,
            name: "Phase 3 current",
            unit_symbol: "A",
            unit_name: "Amps",
        }),
        InputRegister::Phase1Power => Box::new(InputRegisterValue {
            address: 0x000C,
            name: "Phase 1 power",
            unit_symbol: "W",
            unit_name: "Watts",
        }),
        InputRegister::Phase2Power => Box::new(InputRegisterValue {
            address: 0x000E,
            name: "Phase 2 power",
            unit_symbol: "W",
            unit_name: "Watts",
        }),
        InputRegister::Phase3Power => Box::new(InputRegisterValue {
            address: 0x0010,
            name: "Phase 3 power",
            unit_symbol: "W",
            unit_name: "Watts",
        }),
        InputRegister::Phase1ApparentPower => Box::new(InputRegisterValue {
            address: 0x0012,
            name: "Phase 1 apparent power",
            unit_symbol: "VA",
            unit_name: "Volt-Amps",
        }),
        InputRegister::Phase2ApparentPower => Box::new(InputRegisterValue {
            address: 0x0014,
            name: "Phase 2 apparent power",
            unit_symbol: "VA",
            unit_name: "Volt-Amps",
        }),
        InputRegister::Phase3ApparentPower => Box::new(InputRegisterValue {
            address: 0x0016,
            name: "Phase 3 apparent power",
            unit_symbol: "VA",
            unit_name: "Volt-Amps",
        }),
        InputRegister::Phase1ReactivePower => Box::new(InputRegisterValue {
            address: 0x0018,
            name: "Phase 1 reactive power",
            unit_symbol: "VAr",
            unit_name: "Volt-Amps reactive",
        }),
        InputRegister::Phase2ReactivePower => Box::new(InputRegisterValue {
            address: 0x001A,
            name: "Phase 2 reactive power",
            unit_symbol: "VAr",
            unit_name: "Volt-Amps reactive",
        }),
        InputRegister::Phase3ReactivePower => Box::new(InputRegisterValue {
            address: 0x001C,
            name: "Phase 3 reactive power",
            unit_symbol: "VAr",
            unit_name: "Volt-Amps reactive",
        }),
        InputRegister::Phase1PowerFactor => Box::new(InputRegisterValue {
            address: 0x001E,
            name: "Phase 1 power factor",
            unit_symbol: "",
            unit_name: "",
        }),
        InputRegister::Phase2PowerFactor => Box::new(InputRegisterValue {
            address: 0x0020,
            name: "Phase 2 power factor",
            unit_symbol: "",
            unit_name: "",
        }),
        InputRegister::Phase3PowerFactor => Box::new(InputRegisterValue {
            address: 0x0022,
            name: "Phase 3 power factor",
            unit_symbol: "",
            unit_name: "",
        }),
        InputRegister::Phase1PhaseAngle => Box::new(InputRegisterValue {
            address: 0x0024,
            name: "Phase 1 phase angle",
            unit_symbol: "°",
            unit_name: "Degrees",
        }),
        InputRegister::Phase2PhaseAngle => Box::new(InputRegisterValue {
            address: 0x0026,
            name: "Phase 2 phase angle",
            unit_symbol: "°",
            unit_name: "Degrees",
        }),
        InputRegister::Phase3PhaseAngle => Box::new(InputRegisterValue {
            address: 0x0028,
            name: "Phase 3 phase angle",
            unit_symbol: "°",
            unit_name: "Degrees",
        }),
        InputRegister::AvgLineToNeutralVoltage => Box::new(InputRegisterValue {
            address: 0x002A,
            name: "Average line to neutral voltage",
            unit_symbol: "V",
            unit_name: "Volts",
        }),
        InputRegister::AvgLineToNeutralCurrent => Box::new(InputRegisterValue {
            address: 0x002E,
            name: "Average line to neutral current",
            unit_symbol: "A",
            unit_name: "Amps",
        }),
        InputRegister::SumOfLineCurrents => Box::new(InputRegisterValue {
            address: 0x0030,
            name: "Sum of line currents",
            unit_symbol: "A",
            unit_name: "Amps",
        }),
        InputRegister::TotalSystemPower => Box::new(InputRegisterValue {
            address: 0x0034,
            name: "Total system power",
            unit_symbol: "W",
            unit_name: "Watts",
        }),
        InputRegister::FrequencyVoltageLines => Box::new(InputRegisterValue {
            address: 0x0046,
            name: "Frequency voltage",
            unit_symbol: "Hz",
            unit_name: "Hertz",
        }),
    }
}
