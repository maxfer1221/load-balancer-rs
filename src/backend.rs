pub struct Backend {
    ip: String,
}

impl Backend {
    pub fn new(ip: String) -> Self {
        Backend { ip: ip }
    }

    pub fn get_ip(&self) -> &String { &self.ip }
}
