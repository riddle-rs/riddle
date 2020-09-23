pub trait CloneHandle {
    type Handle: std::ops::Deref<Target = Self>;
    type WeakHandle;

    fn clone_handle(&self) -> Self::Handle;
    fn clone_weak_handle(&self) -> Self::WeakHandle;
}

#[macro_export]
macro_rules! define_handles {
    (< $t:ty > :: $i:ident , $sv:vis $s:ident , $wv:vis $w:ident) => {
        impl riddle_common::CloneHandle for $t {
            type Handle = $s;
            type WeakHandle = $w;

            #[inline]
            fn clone_handle(&self) -> $s {
                <$w>::upgrade(&self.$i).unwrap()
            }

            #[inline]
            fn clone_weak_handle(&self) -> $w {
                self.$i.clone()
            }
        }

        #[derive(Clone)]
        $sv struct $s {
            handle: std::sync::Arc<$t>,
        }

        impl $s {
            #[inline]
            pub fn downgrade(this: &$s) -> $w {
                $w {
                    handle: std::sync::Arc::downgrade(&this.handle)
                }
            }

            #[inline]
            pub fn new<F: FnOnce($w) -> $t>(f: F) -> $s {
                $s {
                    handle: std::sync::Arc::new_cyclic(|weak_sync| {
                        let weak_self = $w { handle: weak_sync.clone() };
                        f(weak_self)
                    })
                }
            }

            #[inline]
            pub fn eq(a: &$s, b: &$s) -> bool {
                std::sync::Arc::ptr_eq(&a.handle, &b.handle)
            }
        }

        impl std::ops::Deref for $s {
            type Target = $t;

            #[inline]
            fn deref(&self) -> &$t {
                std::ops::Deref::deref(&self.handle)
            }
        }

        #[derive(Clone)]
        $wv struct $w {
            handle: std::sync::Weak<$t>,
        }

        impl $w {
            #[inline]
            pub fn upgrade(this: &$w) -> Option<$s> {
                std::sync::Weak::upgrade(&this.handle).map(|s| $s { handle: s.clone() })
            }
        }
    };
}
