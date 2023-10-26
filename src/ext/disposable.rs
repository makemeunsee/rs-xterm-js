use crate::interface;
use crate::Disposable;
use core::ops::{Deref, DerefMut};

interface! {
    #[allow(clippy::module_name_repetitions)]
    pub trait XtermDisposable mirrors Disposable {
        /// Disposes of the instance.
        ///
        /// This can involve unregistering an event listener or cleaning up
        /// resources or anything else that should happen when an instance is
        /// disposed of.
        fn dispose(&self);
    }
}

/// A wrapper for [`Disposable`] that calls [`dispose`] on [`Drop`].
///
/// [`dispose`]: XtermDisposable::dispose
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::module_name_repetitions)]
pub struct DisposableWrapper<D: XtermDisposable> {
    /// The actual [`Disposable`] instance that's being wrapped.
    inner: Option<D>,
}

impl<D: XtermDisposable + Default> Default for DisposableWrapper<D> {
    fn default() -> Self {
        D::default().into()
    }
}

impl<D: XtermDisposable> DisposableWrapper<D> {
    /// Pulls the inner [`XtermDisposable`] implementation (`D`) out of the
    /// wrapper, making it so that [`dispose`] is not called on [`Drop`].
    ///
    /// [`dispose`]: XtermDisposable::dispose
    pub fn manually_dispose(mut self) -> D {
        // Every method we offer (other than the Drop impl) assume that inner
        // will be `Some`.
        //
        // This is fairly easy to show; the only constructor we offer
        // initializes `inner` as `Some` and this is the only method that goes
        // from `Some` to `None`. Because this method takes the wrapper by value
        // (consumes it), this is okay; none of the methods (other than `drop`)
        // can be called on this wrapper after this method is called.
        self.inner.take().unwrap()
    }
}

impl<D: XtermDisposable> From<D> for DisposableWrapper<D> {
    fn from(inner: D) -> Self {
        Self { inner: Some(inner) }
    }
}

impl<D: XtermDisposable> Deref for DisposableWrapper<D> {
    type Target = D;

    fn deref(&self) -> &D {
        self.inner.as_ref().unwrap()
    }
}

impl<D: XtermDisposable> DerefMut for DisposableWrapper<D> {
    fn deref_mut(&mut self) -> &mut D {
        self.inner.as_mut().unwrap()
    }
}

impl<D: XtermDisposable> Drop for DisposableWrapper<D> {
    fn drop(&mut self) {
        if let Some(ref inner) = self.inner {
            inner.dispose();
        }
    }
}
