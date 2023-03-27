use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use dyn_clone::clone_trait_object;
use tokio::sync::mpsc;

use crate::types::{configuration::Service, process::Process};

use super::{
    core::ProcessEvent,
    drivers::{exec, flox, DriverPlugin},
};

clone_trait_object!(DriverPlugin);

#[derive(Clone)]
struct Vertex {
    id: Option<String>,
    name: String,
    r#type: String, // docker, podman, exec, wasm
    command: String,
    stop_command: Option<String>,
    working_dir: String,
    watch_dir: Option<String>,
    description: Option<String>,
    depends_on: Vec<String>,
    dependencies: Vec<String>,
    env: HashMap<String, String>,
    autostart: bool,
    autorestart: bool,
    namespace: Option<String>,
    port: Option<u16>,
    stdout: String,
    stderr: String,
    driver: Box<dyn DriverPlugin + Send>,
}

impl From<&Service> for Vertex {
    fn from(service: &Service) -> Self {
        Self {
            id: service.id.clone(),
            name: service.name.clone(),
            r#type: service.r#type.clone(),
            command: service.command.clone(),
            stop_command: service.stop_command.clone(),
            working_dir: service.working_dir.clone(),
            watch_dir: service.watch_dir.clone(),
            description: service.description.clone(),
            depends_on: service.depends_on.clone(),
            dependencies: service.dependencies.clone(),
            env: service.env.clone(),
            autostart: service.autostart,
            autorestart: service.autorestart,
            namespace: service.namespace.clone(),
            port: service.port,
            stdout: service.stdout.clone(),
            stderr: service.stderr.clone(),
            driver: Box::new(exec::driver::Driver::default()),
        }
    }
}

impl Into<Service> for Vertex {
    fn into(self) -> Service {
        Service {
            id: self.id,
            name: self.name,
            r#type: self.r#type,
            command: self.command,
            stop_command: self.stop_command,
            working_dir: self.working_dir,
            watch_dir: self.watch_dir,
            description: self.description,
            depends_on: self.depends_on,
            dependencies: self.dependencies,
            env: self.env,
            autostart: self.autostart,
            autorestart: self.autorestart,
            namespace: self.namespace,
            port: self.port,
            stdout: self.stdout,
            stderr: self.stderr,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
struct Edge {
    from: usize,
    to: usize,
}

#[derive(Clone)]
pub struct DependencyGraph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    project: String,
}

impl DependencyGraph {
    pub fn new(project: String) -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            project,
        }
    }

    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    pub fn add_vertex(
        &mut self,
        service: &Service,
        processes: Arc<Mutex<Vec<(Process, String)>>>,
        childs: Arc<Mutex<HashMap<String, i32>>>,
        event_tx: mpsc::UnboundedSender<ProcessEvent>,
    ) -> usize {
        let mut vertex = Vertex::from(service);
        vertex.driver = match service.flox.as_ref() {
            Some(_) => Box::new(flox::driver::Driver::new(
                service, processes, event_tx, childs,
            )),
            None => Box::new(exec::driver::Driver::new(
                service, processes, event_tx, childs,
            )),
        };
        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push(Edge { from, to });
    }

    pub fn start_services(&self) {
        let mut visited = vec![false; self.vertices.len()];
        for vertex in self.vertices.clone().into_iter() {
            self.start_service(&vertex.into(), &mut visited);
        }
    }

    pub fn start_service(&self, service: &Service, visited: &mut Vec<bool>) {
        let index = self
            .vertices
            .iter()
            .position(|v| v.name == service.name)
            .unwrap();
        if visited[index] {
            return;
        }
        visited[index] = true;
        for edge in self.edges.iter().filter(|e| e.from == index) {
            let service = self.vertices[edge.to].name.clone();
            self.start_service(
                &Service {
                    name: service,
                    ..Default::default()
                },
                visited,
            );
        }

        println!("Starting service {}", self.vertices[index].name);
        self.vertices[index]
            .driver
            .start(self.project.clone())
            .unwrap();
    }

    pub fn stop_services(&self) {
        let mut visited = vec![false; self.vertices.len()];
        for vertex in self.vertices.clone().into_iter() {
            self.stop_service(&vertex.into(), &mut visited);
        }
    }

    pub fn stop_service(&self, service: &Service, visited: &mut Vec<bool>) {
        let index = self
            .vertices
            .iter()
            .position(|v| v.name == service.name)
            .unwrap();
        if visited[index] {
            return;
        }
        visited[index] = true;
        for edge in self.edges.iter().filter(|e| e.to == index) {
            let service = self.vertices[edge.from].clone().into();
            self.stop_service(&service, visited);
        }

        println!("Stopping service {}", self.vertices[index].name);
        self.vertices[index]
            .driver
            .stop(self.project.clone())
            .unwrap();
    }
}
