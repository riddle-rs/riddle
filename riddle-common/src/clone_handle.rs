/// Trait for objects that contain a weak reference to themselves,
/// to allow functions which get references to those objects to
/// clone handles to those objects.
pub trait CloneHandle {
    /// The type which represents a strong reference, and which
    /// may be dereferenced as Self.
    type Handle: std::ops::Deref<Target = Self>;

    /// The type which represents a weak reference.
    type WeakHandle;

    /// Clone a strong handle to the object.
    ///
    /// # Panic
    ///
    /// Panics if the weak reference is invalid. Should only happen
    /// if a handle is being cloned during the Drop::drop method
    /// for self.
    fn clone_handle(&self) -> Self::Handle;

    /// Clone a weak handle to the object.
    fn clone_weak_handle(&self) -> Self::WeakHandle;
}

/// Implement CloneHandle trait and define handle types for a given object.
///
/// # Example
///
/// ```
/// # #![feature(arc_new_cyclic)]
/// # use riddle_common::*;
/// struct SimpleStruct {
///     weak_self: SimpleStructWeak
/// }
/// define_handles!(<SimpleStruct>::weak_self, SimpleStructHandle, SimpleStructWeak);
///
/// fn main() {
///     let handle: SimpleStructHandle = SimpleStructHandle::new(|weak_self| SimpleStruct {
///         weak_self
///     });
/// }
/// ```
///
/// # Generic Example
///
/// ```
/// # #![feature(arc_new_cyclic)]
/// # use riddle_common::*;
/// struct GenericStruct<T: Clone> {
///     weak_self: GenericStructWeak<T>,
///     value: T,
/// }
/// define_handles!(<GenericStruct<T> where T: Clone>::weak_self, GenericStructHandle<T>, GenericStructWeak<T>);
///
/// fn main() {
///     let handle: GenericStructHandle<bool> = GenericStructHandle::new(|weak_self| GenericStruct {
///         weak_self,
///         value: true,
///     });
/// }
/// ```
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
            /// Downgrade this handle to a weak handle
            #[inline]
            pub fn downgrade(this: &$s) -> $w {
                $w {
                    handle: std::sync::Arc::downgrade(&this.handle)
                }
            }

            /// Instantiate a new instance of the underlying object. A copy of
            /// the weak reference is passed to the closure with which to construct
            /// the object
            #[inline]
            pub(crate) fn new<F: FnOnce($w) -> $t>(f: F) -> $s {
                $s {
                    handle: std::sync::Arc::new_cyclic(|weak_sync| {
                        let weak_self = $w { handle: weak_sync.clone() };
                        f(weak_self)
                    })
                }
            }

            /// Test whether two handles point to the same location in memory
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
            /// Upgrade a weak handle to a strong handle. Returns None if the weak
            /// reference no longer points to a live object
            #[inline]
            pub fn upgrade(this: &$w) -> Option<$s> {
                std::sync::Weak::upgrade(&this.handle).map(|s| $s { handle: s.clone() })
            }
        }
    };
    (< $t:ident<T> where T: $ta:ident > :: $i:ident , $sv:vis $s:ident<T> , $wv:vis $w:ident<T>) => {
        impl<T: $ta> riddle_common::CloneHandle for $t<T> {
            type Handle = $s<T>;
            type WeakHandle = $w<T>;

            #[inline]
            fn clone_handle(&self) -> $s<T> {
                <$w<T>>::upgrade(&self.$i).unwrap()
            }

            #[inline]
            fn clone_weak_handle(&self) -> $w<T> {
                self.$i.clone()
            }
        }

        $sv struct $s<T: $ta> {
            handle: std::sync::Arc<$t<T>>,
        }

        impl<T: $ta> $s<T> {
            /// Downgrade this handle to a weak handle
            #[inline]
            pub fn downgrade(this: &Self) -> $w<T> {
                $w {
                    handle: std::sync::Arc::downgrade(&this.handle)
                }
            }

            /// Instantiate a new instance of the underlying object. A copy of
            /// the weak reference is passed to the closure with which to construct
            /// the object
            #[inline]
            pub(crate) fn new<F: FnOnce($w<T>) -> $t<T>>(f: F) -> Self {
                $s {
                    handle: std::sync::Arc::new_cyclic(|weak_sync| {
                        let weak_self = $w { handle: weak_sync.clone() };
                        f(weak_self)
                    })
                }
            }

            /// Test whether two handles point to the same location in memory
            #[inline]
            pub fn eq(a: &Self, b: &Self) -> bool {
                std::sync::Arc::ptr_eq(&a.handle, &b.handle)
            }
        }

        impl<T: $ta> std::ops::Deref for $s<T> {
            type Target = $t<T>;

            #[inline]
            fn deref(&self) -> &$t<T> {
                std::ops::Deref::deref(&self.handle)
            }
        }

        impl<T: $ta> Clone for $s<T> {
            #[inline]
            fn clone(&self) -> Self {
                Self { handle: self.handle.clone() }
            }
        }

        $wv struct $w<T: $ta> {
            handle: std::sync::Weak<$t<T>>,
        }

        impl<T: $ta> $w<T> {
            /// Upgrade a weak handle to a strong handle. Returns None if the weak
            /// reference no longer points to a live object
            #[inline]
            pub fn upgrade(this: &Self) -> Option<$s<T>> {
                std::sync::Weak::upgrade(&this.handle).map(|s| $s { handle: s.clone() })
            }
        }

        impl<T: $ta> Clone for $w<T> {
            #[inline]
            fn clone(&self) -> Self {
                Self { handle: self.handle.clone() }
            }
        }
    };
}
