// server state will have .. ip and port ( we can get that form the siurces.yml , check if it is active (up) and the number of connections )

#[derive(Clone, Debug, Default)]
pub struct Server {
    pub ip: String,
    pub can_connect: bool,
}
