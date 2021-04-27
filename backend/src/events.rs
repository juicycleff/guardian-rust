use riker::actors::ActorSystem;

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    pub static ref SYSTEM: ActorSystem = ActorSystem::new().unwrap();
}

#[cfg(test)]
mod tests {
}
