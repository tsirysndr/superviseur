pub mod driver;
pub mod setup;

macro_rules! create_network {
    ($docker: expr, $network_name: expr) => {
        let config = CreateNetworkOptions {
            name: $network_name.clone(),
            ..Default::default()
        };
        $docker.create_network(config).await?;
    };
}

macro_rules! connect_network {
    ($service_name: expr, $network_name: expr, $container_id: expr, $docker: expr) => {
        let config = ConnectNetworkOptions {
            container: $container_id.clone().unwrap(),
            endpoint_config: EndpointSettings {
                aliases: Some(vec![$service_name.clone()]),
                ..Default::default()
            },
            ..Default::default()
        };
        $docker.connect_network($network_name, config).await?;
    };
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub(crate) use connect_network;
pub(crate) use create_network;
pub(crate) use hashmap;
