use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};

#[allow(dead_code)]
pub struct ChocolateBoiler {
    empty: bool,
    boiled: bool,
}

#[allow(dead_code)]
impl ChocolateBoiler {
    fn new() -> Self {
        ChocolateBoiler {
            empty: true,
            boiled: false,
        }
    }

    pub fn get_instance() -> &'static Mutex<Self> {
        static mut SINGLETON: MaybeUninit<Mutex<ChocolateBoiler>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            let singleton = Self::new();
            unsafe { SINGLETON.as_mut_ptr().write(Mutex::new(singleton)) };
        });

        unsafe { SINGLETON.assume_init_ref() }
    }

    pub fn fill(&mut self) {
        if self.is_empty() {
            println!("Fill the boiler");
            self.empty = false;
            self.boiled = false;
        }
    }
    pub fn drain(&mut self) {
        if !self.is_empty() && self.is_boiled() {
            println!("Drain the boiler");
            self.empty = true;
        }
    }
    pub fn boil(&mut self) {
        if !self.is_empty() && !self.is_boiled() {
            println!("Heat up the boiler");
            self.boiled = true
        }
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }
    pub fn is_boiled(&self) -> bool {
        self.boiled
    }
}
