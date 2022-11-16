use strum::IntoEnumIterator;
mod input_register;
use input_register::{generate_input_register, InputRegister};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio_serial::SerialStream;

    use tokio_modbus::prelude::*;

    let tty_path = "/dev/serial0";
    let slave = Slave(0x01);

    let builder = tokio_serial::new(tty_path, 38400)
        .data_bits(tokio_serial::DataBits::Eight)
        .parity(tokio_serial::Parity::None)
        .stop_bits(tokio_serial::StopBits::One);

    let port = SerialStream::open(&builder).unwrap();

    let mut ctx = rtu::connect_slave(port, slave).await?;
    let mut registers = Vec::new();
    for reg in InputRegister::iter() {
        registers.push(generate_input_register(reg));
    }
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));
    loop {
        interval.tick().await;
        for reg in &registers {
            let rsp = ctx.read_input_registers(reg.address, 2).await.unwrap();
            let var = rsp
                .iter()
                .copied()
                .map(u16::to_be_bytes)
                .flat_map(IntoIterator::into_iter)
                .collect::<Vec<u8>>();
            let val = f32::from_be_bytes(var[..4].try_into().unwrap());
            let msg = format!(
                "{name} is {value}{unit_symbol}",
                name = reg.name,
                value = val,
                unit_symbol = reg.unit_symbol
            );
            println!("{} ", msg);
        }
    }
    #[allow(unreachable_code)]
    {
        Ok(())
    }
}
