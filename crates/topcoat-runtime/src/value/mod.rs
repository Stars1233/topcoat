pub mod str;
pub mod string;

pub trait Value {
    type Surrogate: ?Sized;

    fn ref_cast(&self) -> &Self::Surrogate;
}
