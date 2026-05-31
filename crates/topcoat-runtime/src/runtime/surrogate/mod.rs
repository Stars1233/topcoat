mod _f64;
mod signal;

pub use _f64::*;
pub use signal::*;

pub trait Surrogated {
    type Surrogate;

    fn into_surrogate(self) -> Self::Surrogate;
    fn from_surrogate(surrogate: Self::Surrogate) -> Self;
}

pub trait ToJs {
    fn to_js(&self, out: &mut String);
}

macro_rules! impl_surrogate {
    (
        $({$($g:tt)*})? $real:ty, $surrogate:ty
        $(where $($w:tt)*)?
    ) => {
        impl<$($($g)*)?> Surrogated for $real
        $(where $($w)*)?
        {
            type Surrogate = $surrogate;

            fn into_surrogate(self) -> Self::Surrogate {
                <$surrogate>::new(self)
            }

            fn from_surrogate(surrogate: Self::Surrogate) -> Self {
                surrogate.0
            }
        }
    };
}
pub(crate) use impl_surrogate;

macro_rules! impl_surrogate_ref {
    (
        $({$($g:tt)*})? $real:ty, $surrogate:ty
        $(where $($w:tt)*)?
    ) => {
        impl<'__lifetime, $($($g)*)?> Surrogated for &'__lifetime $real
        $(where $($w)*)?
        {
            type Surrogate = &'__lifetime $surrogate;

            fn into_surrogate(self) -> Self::Surrogate {
                <$surrogate>::ref_cast(self)
            }

            fn from_surrogate(surrogate: Self::Surrogate) -> Self {
                &surrogate.0
            }
        }
    };
}
pub(crate) use impl_surrogate_ref;

macro_rules! impl_surrogate_mut {
    (
        $({$($g:tt)*})? $real:ty, $surrogate:ty
        $(where $($w:tt)*)?
    ) => {
        impl<'__lifetime, $($($g)*)?> Surrogated for &'__lifetime mut $real
        $(where $($w)*)?
        {
            type Surrogate = &'__lifetime mut $surrogate;

            fn into_surrogate(self) -> Self::Surrogate {
                <$surrogate>::ref_cast_mut(self)
            }

            fn from_surrogate(surrogate: Self::Surrogate) -> Self {
                &mut surrogate.0
            }
        }
    };
}
pub(crate) use impl_surrogate_mut;
