pub(crate) enum Event {
    ProxyStatus,
    ProxyEvent
}

impl Event {
    pub(crate) fn to_string(&self) -> String {
        match self {
            Event::ProxyStatus => String::from("ProxyStatus"),
            Event::ProxyEvent => String::from("ProxyEvent"),
        }
    }

}
