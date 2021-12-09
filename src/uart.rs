use gestalt_brick::{BrickBase, CallbackExec, InterruptPlug, InterruptSplitter};
use gestalt_hal_nrf52840::gestalt::gpio::gestalt_reference_api::generic::GenericToUsize;
use gestalt_hal_nrf52840::gestalt::gpio::GpioPin;
use gestalt_reference_api::interface::GenericInterface;
use gestalt_hal_nrf52840::gestalt::uart;
use gestalt_hal_nrf52840::gestalt::uart::{Baudrate, Uart, UartFullEvents, UartInst};
use gestalt_reference_api::generic::GenericEvent;
use gestalt_reference_api::uart::{GenericUartStatus, GenericUartInterrupt, FullGenericUartInterrupt};
use gestalt_reference_api::uart::GenericUartInterrupt::Rx;
use nrf52840_pac::interrupt;


fn uart_event_bind(uart_x: & mut UartBrick<BrickBase<GenericUartInterrupt, GenericUartStatus, Uart, 2>>)
{
    uart_x.base.status = GenericUartStatus::Idle;

    if uart_x.base.data.is_event_active(UartFullEvents::Endrx)
    {
        uart_x.base.data.flush_event(UartFullEvents::Endrx);
        uart_x.base.output_interrupt[GenericUartInterrupt::Rx as usize].splitter_ouput_interrupt.iter_mut().map(
            |this| this.call()
        );
    }
    else if uart_x.base.data.is_event_active(UartFullEvents::Endtx)
    {
        uart_x.base.data.is_event_active(UartFullEvents::Endtx);
        uart_x.base.output_interrupt[GenericUartInterrupt::Tx as usize].splitter_ouput_interrupt.iter_mut().map(
            |this| this.call()
        );

    }
}

#[no_mangle]
unsafe  fn UARTE0_UART0() { uart_event_bind(&mut UART_0); }

#[no_mangle]
unsafe fn UARTE1() { uart_event_bind(&mut UART_1) }

struct qwer{x: usize}
struct tyui{y: &'static str}

static mut UART_0: UartBrick<BrickBase<GenericUartInterrupt, GenericUartStatus, Uart, 2>> = _new(UartInst::Uart0);
static mut UART_1: UartBrick<BrickBase<GenericUartInterrupt, GenericUartStatus, Uart, 2>> = _new(UartInst::Uart0);

pub struct UartBrick<T>
{
    pub base: T
}

impl <'a> gestalt_reference_api::uart::GenericUart for UartBrick<BrickBase<'a, GenericUartInterrupt, GenericUartStatus, Uart, 2>>
{
    type TxPin	= GpioPin;
    type RxPin	= GpioPin;
    type Baud	= Baudrate;
    type TxBuf	= usize;
    type RxBuf	= usize;

    fn set_rx       (&self, pin: Self::RxPin) {
        self.base.data.set_rx(pin);
    }
    fn set_tx       (&self, pin: Self::TxPin) {
        self.base.data.set_tx(pin);
    }
    fn set_baud     (&self, pin: Self::Baud) {
        self.base.data.set_baud(pin);
    }
    fn set_tx_buf   (&self, buf: &[Self::TxBuf]) {
        self.base.data.set_tx_buf(buf);
    }
    fn set_rx_buf   (&self, buf: &[Self::RxBuf]) {
        self.base.data.set_rx_buf(buf);
    }
}

impl <'a>gestalt_reference_api::interface::GenericInterface for
UartBrick<BrickBase<'a, GenericUartInterrupt, GenericUartStatus, Uart, 2>>
{
    fn write(&mut self) { self.base.data.write(); }
    fn read(&mut self)  { self.base.data.write(); }
}

impl <'a> gestalt_brick::BrickExternImpl for
UartBrick<BrickBase<'a, GenericUartInterrupt, GenericUartStatus, Uart, 2>> {
    fn brick_main(&mut self)
    {
        match self.base.status
        {
            GenericUartStatus::Idle => {}
            GenericUartStatus::Init => {}
            GenericUartStatus::Write => {}
            GenericUartStatus::Read => {}
        };
    }

    fn poll(&mut self) { self.brick_main(); }
}

fn new <'a> (inst: UartInst) ->
& 'static mut UartBrick<BrickBase<'a, GenericUartInterrupt, GenericUartStatus, Uart, 2>>
{
    unsafe
        {
            match inst
            {
                UartInst::Uart0 => { & mut UART_0 }
                UartInst::Uart1 => { & mut UART_1 }
            }
        }
}

const fn _new <'a> (inst: UartInst) -> UartBrick<BrickBase<'a, GenericUartInterrupt, GenericUartStatus, Uart, 2>>
{
    UartBrick
    {
        base: BrickBase
        {
            status: GenericUartStatus::Init,
            data: uart::new(inst),
            output_interrupt: [InterruptSplitter{ splitter_ouput_interrupt: &mut [] }, InterruptSplitter{ splitter_ouput_interrupt: &mut [] }]
        }
    }
}