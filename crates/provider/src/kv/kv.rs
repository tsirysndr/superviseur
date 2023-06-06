use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Error;
use indexmap::IndexMap;

use crate::kv::macros::*;
use superviseur_types::configuration::Build;
use superviseur_types::configuration::ConfigurationData;
use superviseur_types::configuration::DockerNetworkConfig;
use superviseur_types::configuration::DockerVolumeConfig;
use superviseur_types::configuration::DriverConfig;
use superviseur_types::configuration::RuntimeConfig;
use superviseur_types::configuration::Service;

use super::new_store;
use super::KVPair;
use super::Store;
use super::StoreConfig;

pub const ROOT_KEY: &str = "superviseur";

pub struct Provider {
    kv_client: Box<dyn Store + Send + Sync>,
    root_key: String,
}

impl Default for Provider {
    fn default() -> Self {
        Provider {
            kv_client: Box::new(super::inmemory::store::InMemory::new(StoreConfig::default())),
            root_key: ROOT_KEY.to_string(),
        }
    }
}

impl Provider {
    pub fn init(
        &mut self,
        kv_type: &str,
        endpoints: Vec<String>,
        config: StoreConfig,
    ) -> Result<(), Error> {
        self.kv_client = self
            .create_kv_client(kv_type, endpoints, config)
            .map_err(|e| anyhow!("Failed to connect to KV Store: {}", e))?;
        Ok(())
    }

    pub fn provide(&self) {}

    pub fn watch_kv(&self) {}

    pub fn project_exists(&self, project_id: &str) -> Result<bool, Error> {
        let kv_pairs = self
            .kv_client
            .list(&format!("{}/{}", self.root_key, project_id))?;
        Ok(!kv_pairs.is_empty())
    }

    pub fn build_configuration(&self, project_id: &str) -> Result<ConfigurationData, Error> {
        let root_key = format!("{}/{}", self.root_key, project_id);
        let kv_pairs = self.kv_client.list(&root_key)?;
        let mut config = ConfigurationData::default();
        config.project = get_project!(kv_pairs);

        config.services = self.decode_services(kv_pairs.clone(), &root_key)?;
        config.services.sort_keys();

        if let Ok(network_settings) = self.decode_network_settings(kv_pairs.clone(), &root_key) {
            config.network_settings = Some(network_settings);
        }

        if let Ok(volume_settings) = self.decode_volume_settings(kv_pairs, &root_key) {
            config.volume_settings = Some(volume_settings);
        }

        Ok(config)
    }

    pub fn decode_services(
        &self,
        kv_pairs: Vec<KVPair>,
        root_key: &str,
    ) -> Result<IndexMap<String, Service>, Error> {
        let kv_pairs = kv_pairs
            .into_iter()
            .filter(|kv_pair| kv_pair.key.starts_with(&format!("{}/services", root_key)))
            .collect::<Vec<KVPair>>();
        let mut services = IndexMap::new();
        let mut drivers = IndexMap::new();
        let mut runtimes = IndexMap::new();

        decode_name!(kv_pairs, services);
        decode_optional_param!(kv_pairs, services, "/id", id);
        decode_param!(kv_pairs, services, "/type", r#type);
        decode_command!(kv_pairs, services);
        decode_param!(kv_pairs, services, "/working_dir", working_dir);
        decode_optional_param!(kv_pairs, services, "/description", description);
        decode_optional_param!(kv_pairs, services, "/stop_command", stop_command);
        decode_optional_param!(kv_pairs, services, "/watch_dir", watch_dir);
        decode_vec_param!(kv_pairs, services, "/depends_on", depends_on);
        decode_vec_param!(kv_pairs, services, "/dependencies", dependencies);
        decode_env!(kv_pairs, services);
        decode_optional_bool!(kv_pairs, services, "/autostart", autostart);
        decode_optional_bool!(kv_pairs, services, "/autorestart", autorestart);
        decode_optional_param!(kv_pairs, services, "/namespace", namespace);
        decode_optional_u32!(kv_pairs, services, "/port", port);
        decode_optional_param!(kv_pairs, services, "/stdout", stdout);
        decode_optional_param!(kv_pairs, services, "/stderr", stderr);
        decode_optional_vec_param!(kv_pairs, services, "/wait_for", wait_for);
        decode_optional_build!(kv_pairs, services, "/build/command", build);
        decode_driver_name!(kv_pairs, drivers, services);
        decode_driver_optional_param!(kv_pairs, drivers, "/image", image, services);
        decode_driver_optional_vec_param!(kv_pairs, drivers, "/packages", packages, services);
        decode_driver_optional_param!(kv_pairs, drivers, "/environment", environment, services);
        decode_driver_optional_vec_param!(kv_pairs, drivers, "/volumes", volumes, services);
        decode_driver_optional_vec_param!(kv_pairs, drivers, "/ports", ports, services);
        decode_driver_optional_vec_param!(kv_pairs, drivers, "/networks", networks, services);
        decode_driver_wasm_runtime!(kv_pairs, runtimes, services, drivers);
        decode_wasm_runtime_optional_param!(kv_pairs, runtimes, "/from", from, services, drivers);

        Ok(services)
    }

    fn decode_network_settings(
        &self,
        kv_pairs: Vec<KVPair>,
        root_key: &str,
    ) -> Result<IndexMap<String, DockerNetworkConfig>, Error> {
        let kv_pairs = kv_pairs
            .into_iter()
            .filter(|kv_pair| kv_pair.key.starts_with(&format!("{}/networks", root_key)))
            .collect::<Vec<KVPair>>();

        let mut network_settings = IndexMap::new();

        for kv_pair in kv_pairs
            .clone()
            .into_iter()
            .filter(|kv_pair| kv_pair.key.ends_with("/networks"))
        {
            kv_pair.value.split(",").for_each(|network| {
                network_settings.insert(network.to_string(), DockerNetworkConfig::default());
            });
        }

        for kv_pair in kv_pairs
            .into_iter()
            .filter(|kv_pair| kv_pair.key.ends_with("/driver"))
        {
            let network = kv_pair.key.split("/").nth(3).unwrap();
            network_settings
                .get_mut(network)
                .unwrap()
                .driver
                .replace(kv_pair.value);
        }
        Ok(network_settings)
    }

    fn decode_volume_settings(
        &self,
        kv_pairs: Vec<KVPair>,
        root_key: &str,
    ) -> Result<IndexMap<String, DockerVolumeConfig>, Error> {
        let kv_pairs = kv_pairs
            .into_iter()
            .filter(|kv_pair| kv_pair.key.starts_with(&format!("{}/volumes", root_key)))
            .collect::<Vec<KVPair>>();
        let mut volume_settings = IndexMap::new();
        for kv_pair in kv_pairs
            .into_iter()
            .filter(|kv_pair| kv_pair.key.ends_with("/volumes"))
        {
            kv_pair.value.split(",").for_each(|volume| {
                volume_settings.insert(volume.to_string(), DockerVolumeConfig::default());
            });
        }
        Ok(volume_settings)
    }

    pub fn project_context(&self, project_id: &str) -> Result<String, Error> {
        let context = self
            .kv_client
            .get(&format!("{}/{}/context", self.root_key, project_id))?;
        Ok(context.value)
    }

    pub fn all_projects(&self) -> Result<Vec<(String, String, String)>, Error> {
        let kv_pairs = self.kv_client.list(&self.root_key)?;
        let projects = kv_pairs
            .iter()
            .filter(|kv_pair| kv_pair.key.ends_with("/project"))
            .map(|kv_pair| {
                let project_id = kv_pair.key.split("/").nth(1).unwrap().to_string();
                let context = self
                    .kv_client
                    .get(&format!("{}/{}/context", self.root_key, project_id))
                    .unwrap()
                    .value;
                (project_id, kv_pair.value.clone(), context)
            })
            .collect::<Vec<(String, String, String)>>();

        Ok(projects)
    }

    pub fn project(&self, project_id: &str) -> Result<(String, String), Error> {
        let project = self
            .kv_client
            .get(&format!("{}/{}/project", self.root_key, project_id))?;
        let context = self
            .kv_client
            .get(&format!("{}/{}/context", self.root_key, project_id))?;
        Ok((project.value, context.value))
    }

    pub fn save_configuration(
        &self,
        project_id: &str,
        config: ConfigurationData,
    ) -> Result<Vec<KVPair>, Error> {
        reset_project!(self, project_id);
        save_project!(self, project_id, config.project);
        save_project_context!(self, project_id, config.context);

        for (name, service) in config.services {
            save_service_name!(self, project_id, name);
            save_service_optional_param!(self, project_id, name, service.id, "id");
            save_service_param!(self, project_id, name, service.r#type, "type");
            save_service_param!(self, project_id, name, service.command, "command");
            save_service_param!(self, project_id, name, service.working_dir, "working_dir");
            save_service_optional_param!(
                self,
                project_id,
                name,
                service.description,
                "description"
            );
            save_service_optional_param!(
                self,
                project_id,
                name,
                service.stop_command,
                "stop_command"
            );
            save_service_optional_param!(self, project_id, name, service.watch_dir, "watch_dir");
            save_service_vec_param!(self, project_id, name, service.depends_on, "depends_on");
            save_service_vec_param!(self, project_id, name, service.dependencies, "dependencies");
            save_service_env!(self, project_id, name, service.env);
            save_service_optional_bool_param!(
                self,
                project_id,
                name,
                service.autostart,
                "autostart"
            );
            save_service_optional_bool_param!(
                self,
                project_id,
                name,
                service.autorestart,
                "autorestart"
            );
            save_service_optional_param!(self, project_id, name, service.namespace, "namespace");
            save_service_optional_u32!(self, project_id, name, service.port, "port");
            save_service_optional_param!(self, project_id, name, service.stdout, "stdout");
            save_service_optional_param!(self, project_id, name, service.stderr, "stderr");
            save_service_optional_vec_param!(self, project_id, name, service.wait_for, "wait_for");
            save_service_build_command!(self, project_id, name, service.build);

            if let Some(r#use) = service.r#use {
                for (driver, driver_config) in r#use {
                    save_service_driver_name!(self, project_id, name, driver);
                    save_service_driver_environment!(self, project_id, name, driver, driver_config);
                    save_service_driver_packages!(self, project_id, name, driver, driver_config);
                    save_service_driver_image!(self, project_id, name, driver, driver_config);
                    save_service_driver_volumes!(self, project_id, name, driver, driver_config);
                    save_service_driver_ports!(self, project_id, name, driver, driver_config);
                    save_service_driver_networks!(self, project_id, name, driver, driver_config);
                    if let Some(runtime) = driver_config.runtime {
                        let (runtime, runtime_config) = runtime.into_iter().next().unwrap();
                        save_service_wasm_runtime_name!(self, project_id, name, driver, runtime);
                        save_service_wasm_spin_from!(
                            self,
                            project_id,
                            name,
                            driver,
                            runtime,
                            runtime_config.from
                        );
                    }
                }
            }

            /*
            TODO: Implement deploy
            TODO: Implement test
            */
        }

        if let Some(network_settings) = config.network_settings {
            save_networks!(
                self,
                project_id,
                network_settings.keys().cloned().collect::<Vec<String>>()
            );
            for (network, network_config) in network_settings {
                save_network_driver!(self, project_id, network, network_config.driver);
            }
        }

        if let Some(volume_settings) = config.volume_settings {
            save_volumes!(
                self,
                project_id,
                volume_settings.keys().cloned().collect::<Vec<String>>()
            );
        }

        let kv_pairs = self
            .kv_client
            .list(&format!("{}/{}", self.root_key, project_id))?;
        Ok(kv_pairs)
    }

    pub fn delete_configuration(&self, project_id: &str) -> Result<(), Error> {
        self.kv_client
            .delete_tree(&format!("{}/{}", self.root_key, project_id))?;
        Ok(())
    }

    pub fn create_kv_client(
        &self,
        kv_type: &str,
        endpoints: Vec<String>,
        config: StoreConfig,
    ) -> Result<Box<dyn Store + Send + Sync>, Error> {
        new_store(kv_type, endpoints, config)
    }
}
