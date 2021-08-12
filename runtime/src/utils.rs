#[macro_export]
macro_rules! set_panic 
{
    () => {
        ::std::panic::set_hook(
            Box::new(console_error_panic_hook::hook)
        )
    };
}

#[macro_export]
macro_rules! log 
{
    ( $($t:tt)* ) => {
        ::web_sys::console::log_1(
            &format!{
                $( $t )*
            }.into()
        )
    };
}