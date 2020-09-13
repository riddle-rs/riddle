pub trait CloneHandle {
    type Handle;
    type WeakHandle;

    fn clone_handle(&self) -> Option<Self::Handle>;
    fn clone_weak_handle(&self) -> Self::WeakHandle;
}
