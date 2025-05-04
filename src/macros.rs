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
    ($inner: ident,$outer:ident, $ret: ty, $raw_type: ty) => {
        fn $inner<'a, 'b, F, T>(&'a self, f: Option<F>, data: &'a mut T) -> Result<(), UiError>
        where
            T: Copy + 'b,
            F: FnMut(Control<Self>, &'b mut T) -> $ret + Send + 'static,
            'b: 'a,
        {
            static STATE: Mutex<
                Option<HashMap<isize, Box<dyn FnMut(*mut $raw_type, *mut c_void) -> $ret + Send>>>,
            > = Mutex::new(None);

            unsafe extern "C" fn cb_(w: *mut $raw_type, data: *mut c_void) -> $ret {
                match modify_callback!(STATE, c, {
                    c.get_mut(&(w as *const _ as _))
                        .and_then(|f| Some(f(w, data)))
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
                            Box::new(move |w, d| {
                                f(Self::from_ptr(w as _).into(), unsafe {
                                    transmute(d as *mut T)
                                })
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
