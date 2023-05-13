pub mod client;
pub mod grpc;

pub use client::nacos_client::init_global_system_actor;
pub use client::nacos_client::close_global_system_actor as close_current_system;
pub use client::nacos_client::get_last_config_client;
pub use client::nacos_client::get_last_naming_client;
pub use client::nacos_client::ActorCreate;
pub use client::nacos_client::ActorCreateWrap;
pub use client::nacos_client::ActixSystemCreateCmd;
pub use client::nacos_client::ActixSystemCreateAsyncCmd;