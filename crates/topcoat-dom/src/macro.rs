#[macro_export]
macro_rules! html {
    ($($tt:tt)*) => {{
        ::topcoat::view! { $($tt)* }
    }};
}
