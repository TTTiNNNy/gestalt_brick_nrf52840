use gestalt_brick::{BrickBase, InterruptSplitter};
use gestalt_hal_nrf52840::gestalt::gpio;
use gestalt_hal_nrf52840::gestalt::gpio::{GpioFullEvents, GpioInst, GpioPin, GpioState, Port};
use gestalt_reference_api::generic::GenericEvent;
use gestalt_reference_api::gpio::{GenericGpioInterrupt, GenericGpioStatus};

const INTERRUPT_NUMBER: usize = 8;
const PORT_NUMBER: usize = 2;


fn gpio_event_bind(gpio_x: & mut GpioBrick<BrickBase<GenericGpioInterrupt, GenericGpioStatus, Port, INTERRUPT_NUMBER>>)
{
    let interrupt_numb = gpio_x.base.data.witch_interrupt_number_active().unwrap();
    unsafe
        {
            GPIO_ARRAY[GPIO_INTERRUPT_ARRAY[interrupt_numb].port_number].base.output_interrupt[interrupt_numb].splitter_ouput_interrupt.iter_mut().map
            (|this| this.call());
        }
    gpio_x.base.data.flush_all_events();
}

#[no_mangle]
unsafe  fn GPIOTE() { gpio_event_bind(&mut GPIO_ARRAY[0]); }




static mut GPIO_ARRAY: [ GpioBrick<BrickBase<GenericGpioInterrupt, GenericGpioStatus, Port, INTERRUPT_NUMBER>>; PORT_NUMBER] = [_new(GpioInst::P0), _new(GpioInst::P1)];
// static mut GPIO_INTERRUPT_ARRAY: [GpioInterruptCell; 8] =
//     [
//         GpioInterruptCell{
//         port: GpioInst::P0,
//         pin: GpioPin::Gpio0,
//         interrupt_trigger: GenericGpioInterrupt::LoToHi
//     }; 8 ];

static mut GPIO_INTERRUPT_ARRAY: [GpioInterruptCell; INTERRUPT_NUMBER] = [GpioInterruptCell{
    port_number: 0,
    pin: GpioPin::Gpio0,
    interrupt_trigger: GenericGpioInterrupt::LoToHi,
}; INTERRUPT_NUMBER];


#[derive(Copy, Clone)]
struct GpioInterruptCell{
    port_number: usize,
    pin: GpioPin,
    interrupt_trigger: GenericGpioInterrupt,
}

// impl Clone for GpioInterruptCell {
//     fn clone(&self) -> Self {
//         GpioInterruptCell{
//             port: self.port,
//             pin: self.pin,
//             interrupt_trigger: GenericGpioInterrupt::LoToHi
//         }
//     }
// }

pub struct GpioBrickBase
{
    hal_instance: Port,

}

pub struct GpioBrick<T>
{
    pub base: T
}

impl <'a> gestalt_reference_api::gpio::GenericGpio for GpioBrick<BrickBase<'a, GenericGpioInterrupt, GenericGpioStatus, Port, INTERRUPT_NUMBER>>
{
    type Port		= gpio::GpioInst;
    type Pin		= gpio::GpioPin;
    type Dir		= gpio::GpioDir;
    type Pull		= gpio::GpioPull;
    type State		= gpio::GpioState;
    type PortLength	= usize;

    fn set_state(&self, _: Self::Pin, _: Self::State) {
        todo!()
    }

    fn set_high(&self, _: Self::Pin) {
        todo!()
    }

    fn set_low(&self, _: Self::Pin) {
        todo!()
    }

    fn set_direction(&self, _: Self::Pin, _: Self::Dir) {
        todo!()
    }

    fn set_pull(&self, _: Self::Pin, _: Self::Pull) {
        todo!()
    }

    fn set_port(&self) {
        todo!()
    }

    fn set_pull_up(&self, _: Self::Pin) {
        todo!()
    }

    fn set_pull_down(&self, _: Self::Pin) {
        todo!()
    }

    fn set_pull_none(&self, _: Self::Pin) {
        todo!()
    }

    fn get(&self, _: Self::Pin) -> Self::State {
        todo!()
    }

    fn get_port(&self) -> Self::PortLength {
        todo!()
    }

    fn toggle(&self, pin: Self::Pin) {
        self.base.data.toggle(pin);
    }
}

impl <'a> gestalt_brick::BrickExternImpl for
GpioBrick<BrickBase<'a, GenericGpioInterrupt, GenericGpioStatus, Port, INTERRUPT_NUMBER>> {
    fn brick_main(&mut self)
    {
        match self.base.status
        {
            GenericGpioStatus::Idle => {}
            GenericGpioStatus::Init => {}
            GenericGpioStatus::Write => {}
            GenericGpioStatus::Read => {}
        };
    }

    fn poll(&mut self) { self.brick_main(); }
}

fn new <'a> (inst: GpioInst) ->
& 'static mut GpioBrick<BrickBase<'a, GenericGpioInterrupt, GenericGpioStatus, Port, INTERRUPT_NUMBER>>
{
    unsafe
        {
            match inst
            {
                GpioInst::P0 => { & mut GPIO_ARRAY[0] }
                GpioInst::P1 => { & mut GPIO_ARRAY[1] }
            }
        }
}

const fn _new <'a> (inst: GpioInst) -> GpioBrick<BrickBase<'a, GenericGpioInterrupt, GenericGpioStatus, Port, INTERRUPT_NUMBER>>
{
    GpioBrick
    {
        base: BrickBase
        {
            status: GenericGpioStatus::Init,
            data: gpio::new(inst),
            output_interrupt: [
                InterruptSplitter{ splitter_ouput_interrupt: &mut [] },
                InterruptSplitter{ splitter_ouput_interrupt: &mut [] },
                InterruptSplitter{ splitter_ouput_interrupt: &mut [] },
                InterruptSplitter{ splitter_ouput_interrupt: &mut [] },
                InterruptSplitter{ splitter_ouput_interrupt: &mut [] },
                InterruptSplitter{ splitter_ouput_interrupt: &mut [] },
                InterruptSplitter{ splitter_ouput_interrupt: &mut [] },
                InterruptSplitter{ splitter_ouput_interrupt: &mut [] },
            ]
        }
    }
}