use std::sync::Arc;

pub type ValidatorFn<T> = Arc<dyn Fn(&T) -> Result<(), String> + Send + Sync>;
pub type ListenerFn<T> = Arc<dyn Fn(&T) + Send + Sync>;

pub struct OptionInstance<T> {
    pub name: String,
    pub description: Option<String>,
    pub value: T,
    pub default_value: T,
    pub pending_value: Option<T>,
    validators: Vec<ValidatorFn<T>>,
    listeners: Vec<ListenerFn<T>>,
}

impl<T: Clone + PartialEq> OptionInstance<T> {
    pub fn new(name: impl Into<String>, default_value: T) -> Self {
        Self {
            name: name.into(),
            description: None,
            value: default_value.clone(),
            default_value,
            pending_value: None,
            validators: Vec::new(),
            listeners: Vec::new(),
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn add_validator<F>(&mut self, validator: F)
    where
        F: Fn(&T) -> Result<(), String> + Send + Sync + 'static,
    {
        self.validators.push(Arc::new(validator));
    }

    pub fn add_listener<F>(&mut self, listener: F)
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        self.listeners.push(Arc::new(listener));
    }

    pub fn get(&self) -> &T {
        self.pending_value.as_ref().unwrap_or(&self.value)
    }

    pub fn set_pending(&mut self, new_val: T) -> Result<(), String> {
        for v in &self.validators {
            v(&new_val)?;
        }
        self.pending_value = Some(new_val);
        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        match &self.pending_value {
            Some(pending) => pending != &self.value,
            None => false,
        }
    }

    pub fn apply(&mut self) {
        if let Some(pending) = self.pending_value.take() {
            if pending != self.value {
                self.value = pending;
                for listener in &self.listeners {
                    listener(&self.value);
                }
            }
        }
    }

    pub fn reset_to_default(&mut self) {
        self.set_pending(self.default_value.clone()).ok();
    }

    pub fn discard(&mut self) {
        self.pending_value = None;
    }
}
