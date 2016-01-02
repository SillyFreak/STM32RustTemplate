#![macro_use]

use core::ops::{Not, Shl, BitAnd, BitOr, Deref, DerefMut};

pub trait Reg<T> {
    fn mask_set(&mut self, value: T, mask: T);

    fn shift_mask_set<S>(&mut self, value: T, mask: T, shift: S)
            where S: Copy,
                  T: Shl<S, Output=T>;
}

impl<T> Reg<T> for T
        where T: Copy
               + Not<Output=T>
               + BitAnd<Output=T>
               + BitOr<Output=T> {
    fn mask_set(&mut self, value: T, mask: T) {
        *self = (*self & !mask) | (value & mask);
    }

    fn shift_mask_set<S>(&mut self, value: T, mask: T, shift: S)
            where S: Copy,
                  T: Shl<S, Output=T> {
        self.mask_set(value << shift, mask << shift);
    }
}

pub struct RegPtr<T> {
    pub ptr: *mut T,
}

impl<T> RegPtr<T> {
    pub const fn new(addr: usize) -> Self {
        RegPtr { ptr: addr as *mut T, }
    }
}

impl<T> Deref for RegPtr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        return unsafe { & *self.ptr };
    }
}

impl<T> DerefMut for RegPtr<T> {
    fn deref_mut(&mut self) -> &mut T {
        return unsafe { &mut *self.ptr };
    }
}

#[macro_export]
macro_rules! registers {
    (
        $($(#[$Reg_attr:meta])* const $Reg:ident: $T:ty = $value:expr,)+
    ) => {
        $($(#[$Reg_attr])* pub const $Reg: RegPtr<$T> = RegPtr::new($value);)+
    };
}

#[macro_export]
macro_rules! constants {
    (
        $($Mod:ident: $T:ty {
            $($(#[$Const_attr:meta])* const $Const:ident = $value:expr,)+
        })+
    ) => {
        $(#[allow(non_snake_case)]
        pub mod $Mod {
            pub type Type = $T;
            $($(#[$Const_attr])* pub const $Const: Type = $value;)+
        })+
    };
}

