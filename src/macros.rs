#[macro_export]
macro_rules! modify_callback {
    ($var:expr, $def: ident, $block: block) => {
        $var.lock().and_then(move |mut lock| {
            if lock.is_none() {
                *lock = Some(Default::default());
            }

            Ok(lock.as_mut().and_then(move |$def| $block))
        })
    };
}

#[macro_export]
macro_rules! define_callback_function {
    ($inner: ident,$outer:ident, $raw_ret_type: ty, $raw_self_type: ty $(, ($arg_name:ident, $arg_raw_type:ty, $arg_wrapped_type:ty))*) => {
        fn $inner<'a, 'b, F, T>(&'a self, f: Option<F>, data: &'a mut T) -> Result<(), UiError>
        where
            T: Copy + 'b,
            F: FnMut(Self $(, $arg_wrapped_type)*, &'b mut T) -> $raw_ret_type + Send + 'static,
            'b: 'a,
        {
            static STATE: Mutex<
                Option<HashMap<isize, Box<dyn FnMut(*mut $raw_self_type $(, *mut $arg_raw_type)*, *mut c_void) -> $raw_ret_type + Send>>>,
            > = Mutex::new(None);

            unsafe extern "C" fn cb_(w: *mut $raw_self_type $(, $arg_name: *mut $arg_raw_type)*, data: *mut c_void) -> $raw_ret_type {
                match modify_callback!(STATE, c, {
                    c.get_mut(&(w as *const _ as _))
                        .and_then(|f| Some(f(w $(, $arg_name)*, data)))
                }) {
                    Err(e) => {
                        error!("An error was occurred in {}: {}", stringify!($inner), e);
                        Default::default()
                    }
                    Ok(r) => r.unwrap_or_default(),
                }
            }

            let cb = match f {
                Some(mut f) => {
                    modify_callback!(STATE, c, {
                        c.insert(
                            self._inner as _,
                            Box::new(move |w $(, $arg_name)*, d| {
                                let self_ = Self::from_ptr(w as _);
                                unsafe { f(self_ $(, <$arg_wrapped_type>::from_ptr($arg_name as _))*, transmute(d as *mut T)) }
                            }),
                        )
                    })?;
                    Some(cb_ as _)
                }
                _ => {
                    modify_callback!(STATE, c, { c.remove(&(self._inner as _)) })?;
                    None
                }
            };

            Ok(unsafe { $outer(self._inner, cb, transmute(data)) })
        }
    };
}
