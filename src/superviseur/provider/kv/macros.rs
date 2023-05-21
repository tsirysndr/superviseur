macro_rules! save_project {
    ($self:ident, $project_id:ident, $project:expr) => {
        let key = format!("{}/{}/project", $self.root_key, $project_id);

        $self
            .kv_client
            .put(&key, &$project)
            .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
    };
}

macro_rules! save_project_context {
    ($self:ident, $project_id:ident, $context:expr) => {
        let key = format!("{}/{}/context", $self.root_key, $project_id);
        if let Some(context) = $context {
            $self
                .kv_client
                .put(&key, &context)
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_id {
    ($self:ident, $project_id:ident, $name:expr, $id:expr) => {
        let key = format!("{}/{}/services/{}/id", $self.root_key, $project_id, $name);
        if let Some(id) = $id {
            $self
                .kv_client
                .put(&key, &id)
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_name {
    ($self:ident, $project_id:ident, $name:expr) => {
        let key = format!("{}/{}/services/{}/name", $self.root_key, $project_id, $name);
        $self
            .kv_client
            .put(&key, &$name)
            .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
    };
}

macro_rules! save_service_type {
    ($self:ident, $project_id:ident, $name:expr, $type: expr) => {
        let key = format!("{}/{}/services/{}/type", $self.root_key, $project_id, $name);
        $self
            .kv_client
            .put(&key, &$type)
            .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
    };
}

macro_rules! save_service_command {
    ($self:ident, $project_id:ident, $name:ident, $command:expr) => {
        let key = format!(
            "{}/{}/services/{}/command",
            $self.root_key, $project_id, $name
        );
        $self
            .kv_client
            .put(&key, &$command)
            .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
    };
}

macro_rules! save_service_working_dir {
    ($self:ident, $project_id:ident, $name:ident, $working_dir:expr) => {
        let key = format!(
            "{}/{}/services/{}/working_dir",
            $self.root_key, $project_id, $name
        );
        $self
            .kv_client
            .put(&key, &$working_dir)
            .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
    };
}

macro_rules! save_service_description {
    ($self:ident, $project_id:ident, $name:ident, $description:expr) => {
        let key = format!(
            "{}/{}/services/{}/description",
            $self.root_key, $project_id, $name
        );
        if let Some(description) = $description {
            $self
                .kv_client
                .put(&key, &description)
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_stop_command {
    ($self:ident, $project_id:ident, $name:ident, $stop_command:expr) => {
        let key = format!(
            "{}/{}/services/{}/stop_command",
            $self.root_key, $project_id, $name
        );
        if let Some(stop_command) = $stop_command {
            $self
                .kv_client
                .put(&key, &stop_command)
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_watch_dir {
    ($self:ident, $project_id:ident, $name:ident, $watch_dir:expr) => {
        let key = format!(
            "{}/{}/services/{}/watch_dir",
            $self.root_key, $project_id, $name
        );
        if let Some(watch_dir) = $watch_dir {
            $self
                .kv_client
                .put(&key, &watch_dir)
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_depends_on {
    ($self:ident, $project_id:ident, $name:ident, $depends_on:expr) => {
        let key = format!(
            "{}/{}/services/{}/depends_on",
            $self.root_key, $project_id, $name
        );
        $self
            .kv_client
            .put(&key, &$depends_on.join(","))
            .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
    };
}

macro_rules! save_service_dependencies {
    ($self:ident, $project_id:ident, $name:ident, $dependencies:expr) => {
        let key = format!(
            "{}/{}/services/{}/dependencies",
            $self.root_key, $project_id, $name
        );
        $self
            .kv_client
            .put(&key, &$dependencies.join(","))
            .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
    };
}

macro_rules! save_service_env {
    ($self:ident, $project_id:ident, $name:ident, $env:expr) => {
        let key = format!("{}/{}/services/{}/env", $self.root_key, $project_id, $name);
        $self
            .kv_client
            .put(
                &key,
                &$env
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<String>>()
                    .join(","),
            )
            .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
    };
}

macro_rules! save_service_autostart {
    ($self:ident, $project_id:ident, $name:ident, $autostart:expr) => {
        let key = format!(
            "{}/{}/services/{}/autostart",
            $self.root_key, $project_id, $name
        );
        if let Some(autostart) = $autostart {
            $self
                .kv_client
                .put(&key, &autostart.to_string())
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_auto_restart {
    ($self:ident, $project_id:ident, $name:ident, $autorestart:expr) => {
        let key = format!(
            "{}/{}/services/{}/autorestart",
            $self.root_key, $project_id, $name
        );
        if let Some(autorestart) = $autorestart {
            $self
                .kv_client
                .put(&key, &autorestart.to_string())
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_namespace {
    ($self:ident, $project_id:ident, $name:ident, $namespace:expr) => {
        let key = format!(
            "{}/{}/services/{}/namespace",
            $self.root_key, $project_id, $name
        );
        if let Some(namespace) = $namespace {
            $self
                .kv_client
                .put(&key, &namespace)
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_port {
    ($self:ident, $project_id:ident, $name:ident, $port:expr) => {
        let key = format!("{}/{}/services/{}/port", $self.root_key, $project_id, $name);
        if let Some(port) = $port {
            $self
                .kv_client
                .put(&key, &port.to_string())
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_stdout {
    ($self:ident, $project_id:ident, $name:ident, $stdout:expr) => {
        let key = format!(
            "{}/{}/services/{}/stdout",
            $self.root_key, $project_id, $name
        );
        if let Some(stdout) = $stdout {
            $self
                .kv_client
                .put(&key, &stdout)
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_stderr {
    ($self:ident, $project_id:ident, $name:ident, $stderr:expr) => {
        let key = format!(
            "{}/{}/services/{}/stderr",
            $self.root_key, $project_id, $name
        );
        if let Some(stderr) = $stderr {
            $self
                .kv_client
                .put(&key, &stderr)
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_wait_for {
    ($self:ident, $project_id:ident, $name:ident, $wait_for:expr) => {
        let key = format!(
            "{}/{}/services/{}/wait_for",
            $self.root_key, $project_id, $name
        );
        if let Some(wait_for) = $wait_for {
            $self
                .kv_client
                .put(&key, &wait_for.join(","))
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_build_command {
    ($self:ident, $project_id:ident, $name:ident, $build:expr) => {
        let key = format!(
            "{}/{}/services/{}/build/command",
            $self.root_key, $project_id, $name
        );
        if let Some(build) = $build {
            $self
                .kv_client
                .put(&key, &build.command)
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_driver_name {
    ($self:ident, $project_id:ident, $name:ident, $driver:ident) => {
        let key = format!(
            "{}/{}/services/{}/use/{}/name",
            $self.root_key, $project_id, $name, $driver
        );
        $self
            .kv_client
            .put(&key, &$driver)
            .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
    };
}

macro_rules! save_service_driver_environment {
    ($self:ident, $project_id:ident, $name:ident, $driver:ident, $driver_config:expr) => {
        let key = format!(
            "{}/{}/services/{}/use/{}/environment",
            $self.root_key, $project_id, $name, $driver
        );
        if let Some(environment) = $driver_config.environment {
            $self
                .kv_client
                .put(&key, &environment)
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_driver_packages {
    ($self:ident, $project_id:ident, $name:ident, $driver:ident, $driver_config:expr) => {
        let key = format!(
            "{}/{}/services/{}/use/{}/packages",
            $self.root_key, $project_id, $name, $driver
        );
        if let Some(packages) = $driver_config.packages {
            $self
                .kv_client
                .put(&key, &packages.join(","))
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_driver_image {
    ($self:ident, $project_id:ident, $name:ident, $driver:ident, $driver_config:expr) => {
        let key = format!(
            "{}/{}/services/{}/use/{}/image",
            $self.root_key, $project_id, $name, $driver
        );
        if let Some(image) = $driver_config.image {
            $self
                .kv_client
                .put(&key, &image)
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_driver_volumes {
    ($self:ident, $project_id:ident, $name:ident, $driver:ident, $driver_config:expr) => {
        let key = format!(
            "{}/{}/services/{}/use/{}/volumes",
            $self.root_key, $project_id, $name, $driver
        );
        if let Some(volumes) = $driver_config.volumes {
            $self
                .kv_client
                .put(&key, &volumes.join(","))
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_driver_ports {
    ($self:ident, $project_id:ident, $name:ident, $driver:ident, $driver_config:expr) => {
        let key = format!(
            "{}/{}/services/{}/use/{}/ports",
            $self.root_key, $project_id, $name, $driver
        );
        if let Some(ports) = $driver_config.ports {
            $self
                .kv_client
                .put(&key, &ports.join(","))
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_driver_networks {
    ($self:ident, $project_id:ident, $name:ident, $driver:ident, $driver_config:expr) => {
        let key = format!(
            "{}/{}/services/{}/use/{}/networks",
            $self.root_key, $project_id, $name, $driver
        );
        if let Some(networks) = $driver_config.networks {
            $self
                .kv_client
                .put(&key, &networks.join(","))
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_service_wasm_runtime_name {
    ($self:ident, $project_id:ident, $name:ident, $driver:ident, $runtime:ident) => {
        let key = format!(
            "{}/{}/services/{}/use/{}/runtime/name",
            $self.root_key, $project_id, $name, $driver
        );
        $self
            .kv_client
            .put(&key, &$runtime)
            .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
    };
}

macro_rules! save_service_wasm_spin_from {
    ($self:ident, $project_id:ident, $name:ident, $driver:ident, $runtime:ident, $spin_from:expr) => {
        let key = format!(
            "{}/{}/services/{}/use/{}/{}/from",
            $self.root_key, $project_id, $name, $driver, $runtime
        );
        if let Some(spin_from) = $spin_from {
            $self
                .kv_client
                .put(&key, &spin_from)
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! save_volumes {
    ($self:ident, $project_id:ident, $volumes:expr) => {
        let key = format!("{}/{}/volumes", $self.root_key, $project_id);
        $self
            .kv_client
            .put(&key, &$volumes.join(","))
            .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
    };
}

macro_rules! save_networks {
    ($self:ident, $project_id:ident, $networks:expr) => {
        let key = format!("{}/{}/networks", $self.root_key, $project_id);
        $self
            .kv_client
            .put(&key, &$networks.join(","))
            .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
    };
}

macro_rules! save_network_driver {
    ($self:ident, $project_id:ident, $network:ident, $driver:expr) => {
        let key = format!(
            "{}/{}/networks/{}/driver",
            $self.root_key, $project_id, $network
        );
        if let Some(driver) = $driver {
            $self
                .kv_client
                .put(&key, &driver)
                .map_err(|e| anyhow!("Failed to save configuration: {}", e))?;
        }
    };
}

macro_rules! get_project {
    ($kv_pairs:expr) => {
        $kv_pairs
            .iter()
            .find(|kv| kv.key.ends_with("/project"))
            .map(|kv| kv.value.clone())
            .unwrap_or_default()
    };
}

macro_rules! decode_name {
    ($kv_pairs:ident, $services:ident) => {
        for kv_pair in $kv_pairs
            .clone()
            .into_iter()
            .filter(|kv_pair| kv_pair.key.ends_with("/name") && !kv_pair.key.contains("/use/"))
        {
            $services.insert(
                kv_pair.value.clone(),
                Service {
                    name: kv_pair.value,
                    ..Default::default()
                },
            );
        }
    };
}

macro_rules! decode_param {
    ($kv_pairs:ident, $services:ident, $param:expr, $param_name:ident) => {
        for kv_pair in $kv_pairs
            .clone()
            .into_iter()
            .filter(|kv_pair| kv_pair.key.ends_with($param))
        {
            let name = kv_pair.key.split("/").nth(3).unwrap();
            $services.get_mut(name).unwrap().$param_name = kv_pair.value;
        }
    };
}

macro_rules! decode_optional_param {
    ($kv_pairs:ident, $services:ident, $param:expr, $param_name:ident) => {
        for kv_pair in $kv_pairs
            .clone()
            .into_iter()
            .filter(|kv_pair| kv_pair.key.ends_with($param))
        {
            let name = kv_pair.key.split("/").nth(3).unwrap();
            $services
                .get_mut(name)
                .unwrap()
                .$param_name
                .replace(kv_pair.value);
        }
    };
}

macro_rules! decode_command {
    ($kv_pairs:ident, $services:ident) => {
        for kv_pair in $kv_pairs
            .clone()
            .into_iter()
            .filter(|kv_pair| kv_pair.key.ends_with("/command") && !kv_pair.key.contains("/build"))
        {
            let name = kv_pair.key.split("/").nth(3).unwrap();
            $services.get_mut(name).unwrap().command = kv_pair.value;
        }
    };
}

macro_rules! decode_vec_param {
    ($kv_pairs:ident, $services:ident, $param:expr, $param_name:ident) => {
        for kv_pair in $kv_pairs
            .clone()
            .into_iter()
            .filter(|kv_pair| kv_pair.key.ends_with($param))
        {
            if kv_pair.value.is_empty() {
                continue;
            }
            let name = kv_pair.key.split("/").nth(3).unwrap();
            $services.get_mut(name).unwrap().$param_name =
                kv_pair.value.split(",").map(|s| s.to_string()).collect();
        }
    };
}

macro_rules! decode_optional_vec_param {
    ($kv_pairs:ident, $services:ident, $param:expr, $param_name:ident) => {
        for kv_pair in $kv_pairs
            .clone()
            .into_iter()
            .filter(|kv_pair| kv_pair.key.ends_with($param))
        {
            if kv_pair.value.is_empty() {
                continue;
            }
            let name = kv_pair.key.split("/").nth(3).unwrap();
            $services
                .get_mut(name)
                .unwrap()
                .$param_name
                .replace(kv_pair.value.split(",").map(|s| s.to_string()).collect());
        }
    };
}

macro_rules! decode_env {
    ($kv_pairs:ident, $services:ident) => {
        for kv_pair in $kv_pairs
            .clone()
            .into_iter()
            .filter(|kv_pair| kv_pair.key.ends_with("/env"))
        {
            if kv_pair.value.is_empty() {
                continue;
            }
            let name = kv_pair.key.split("/").nth(3).unwrap();
            let env = kv_pair
                .value
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let mut env_map = HashMap::new();
            env.iter().for_each(|kv| {
                let kv = kv.split("=").collect::<Vec<&str>>();
                env_map.insert(kv[0].to_string(), kv[1].to_string());
            });
            $services.get_mut(name).unwrap().env = env_map;
        }
    };
}

macro_rules! decode_optional_bool {
    ($kv_pairs:ident, $services:ident, $param:expr, $param_name:ident) => {
        for kv_pair in $kv_pairs
            .clone()
            .into_iter()
            .filter(|kv_pair| kv_pair.key.ends_with($param))
        {
            let name = kv_pair.key.split("/").nth(3).unwrap();
            $services
                .get_mut(name)
                .unwrap()
                .$param_name
                .replace(kv_pair.value.parse::<bool>().unwrap());
        }
    };
}

macro_rules! decode_optional_u32 {
    ($kv_pairs:ident, $services:ident, $param:expr, $param_name:ident) => {
        for kv_pair in $kv_pairs
            .clone()
            .into_iter()
            .filter(|kv_pair| kv_pair.key.ends_with($param))
        {
            let name = kv_pair.key.split("/").nth(3).unwrap();
            $services
                .get_mut(name)
                .unwrap()
                .$param_name
                .replace(kv_pair.value.parse::<u32>().unwrap());
        }
    };
}

macro_rules! decode_optional_build {
    ($kv_pairs:ident, $services:ident, $param:expr, $param_name:ident) => {
        for kv_pair in $kv_pairs
            .clone()
            .into_iter()
            .filter(|kv_pair| kv_pair.key.ends_with($param))
        {
            let name = kv_pair.key.split("/").nth(3).unwrap();
            $services.get_mut(name).unwrap().$param_name.replace(Build {
                command: kv_pair.value,
            });
        }
    };
}

pub(crate) use decode_command;
pub(crate) use decode_env;
pub(crate) use decode_name;
pub(crate) use decode_optional_bool;
pub(crate) use decode_optional_build;
pub(crate) use decode_optional_param;
pub(crate) use decode_optional_u32;
pub(crate) use decode_optional_vec_param;
pub(crate) use decode_param;
pub(crate) use decode_vec_param;
pub(crate) use get_project;
pub(crate) use save_network_driver;
pub(crate) use save_networks;
pub(crate) use save_project;
pub(crate) use save_project_context;
pub(crate) use save_service_auto_restart;
pub(crate) use save_service_autostart;
pub(crate) use save_service_build_command;
pub(crate) use save_service_command;
pub(crate) use save_service_dependencies;
pub(crate) use save_service_depends_on;
pub(crate) use save_service_description;
pub(crate) use save_service_driver_environment;
pub(crate) use save_service_driver_image;
pub(crate) use save_service_driver_name;
pub(crate) use save_service_driver_networks;
pub(crate) use save_service_driver_packages;
pub(crate) use save_service_driver_ports;
pub(crate) use save_service_driver_volumes;
pub(crate) use save_service_env;
pub(crate) use save_service_id;
pub(crate) use save_service_name;
pub(crate) use save_service_namespace;
pub(crate) use save_service_port;
pub(crate) use save_service_stderr;
pub(crate) use save_service_stdout;
pub(crate) use save_service_stop_command;
pub(crate) use save_service_type;
pub(crate) use save_service_wait_for;
pub(crate) use save_service_wasm_runtime_name;
pub(crate) use save_service_wasm_spin_from;
pub(crate) use save_service_watch_dir;
pub(crate) use save_service_working_dir;
pub(crate) use save_volumes;
