pub trait Binding<T>: Send + Sync {
    fn get(&self) -> T;
    fn set(&self, val: T);
}

pub struct DirectBinding<G, S> {
    getter: G,
    setter: S,
}

impl<G, S> DirectBinding<G, S> {
    pub fn new(getter: G, setter: S) -> Self {
        Self { getter, setter }
    }
}

impl<T, G, S> Binding<T> for DirectBinding<G, S>
where
    T: Clone + Send + Sync + 'static,
    G: Fn() -> T + Send + Sync + 'static,
    S: Fn(T) + Send + Sync + 'static,
{
    fn get(&self) -> T {
        (self.getter)()
    }

    fn set(&self, val: T) {
        (self.setter)(val);
    }
}
