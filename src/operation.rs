use core::cell::{Cell, RefCell};

pub struct Spectrum {
    pub freq: Vec<f32>,
    pub trans: Vec<f32>,
}

impl Spectrum {
    
}

pub trait Operation {
    fn get_spectrum(self: &Self) -> Spectrum;
    fn get_params(self: &Self) -> Vec<f32>;
}

pub struct Add<'a, T: Operation, U: Operation> {
    pub augend: &'a T,
    pub addend: &'a U,
    pub coeff: Cell<f32>,
}

impl<T: Operation, U: Operation> Operation for Add<'_, T, U> {
    fn get_spectrum(self: &Self) -> Spectrum {
        let s1 = self.addend.get_spectrum();
        let s2 = self.augend.get_spectrum();
        let f = s1.freq;
        let t = s1.trans.iter()
                    .zip(s2.trans.iter())
                    .map(|x| x.0 + x.1 * self.coeff.get())
                    .collect();
        Spectrum {freq: f, trans: t}  
    }
    fn get_params(self: &Self) -> Vec<f32> {
        vec![self.coeff.get()]
    }
}

pub struct Generate {
    pub freq: RefCell<Vec<f32>>,
    pub trans: RefCell<Vec<f32>>,
}

impl Operation for Generate {
    fn get_spectrum(self: &Self) -> Spectrum {
        Spectrum{freq: self.freq.borrow().to_vec(), trans: self.trans.borrow().to_vec()}
    }
    fn get_params(self: &Self) -> Vec<f32> {
        vec![]
    }
}

