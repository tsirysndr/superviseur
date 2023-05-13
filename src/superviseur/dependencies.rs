use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use async_recursion::async_recursion;
use dyn_clone::clone_trait_object;
use tokio::sync::mpsc;

use crate::types::{configuration::Service, process::Process};

use super::{
    core::ProcessEvent,
    drivers::{docker, exec, flox, nix, DriverPlugin},
    logs::LogEngine,
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
    port: Option<u32>,
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

#[derive(Debug)]
pub enum GraphCommand {
    AddVertex(
        Service,
        Arc<Mutex<Vec<(Process, String)>>>,
        Arc<Mutex<HashMap<String, i32>>>,
        mpsc::UnboundedSender<ProcessEvent>,
        LogEngine,
    ),
    AddEdge(usize, usize),
    StartService(Service, bool),
    StopService(Service),
    BuildService(Service),
    StopServices,
    BuildServices,
    StartServices(bool),
}

#[derive(Clone)]
pub struct DependencyGraph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    project: String,
    context: String,
    pub cmd_tx: mpsc::UnboundedSender<GraphCommand>,
}

impl DependencyGraph {
    pub fn new(
        project: String,
        context: String,
        cmd_tx: mpsc::UnboundedSender<GraphCommand>,
        cmd_rx: Arc<Mutex<mpsc::UnboundedReceiver<GraphCommand>>>,
    ) -> Self {
        let graph = Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            project,
            context,
            cmd_tx,
        };
        let mut cloned_graph = graph.clone();
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // wait for the first command
                while let Some(cmd) = cmd_rx.lock().unwrap().recv().await {
                    let mut visited = vec![false; cloned_graph.size()];
                    match cmd {
                        GraphCommand::AddVertex(
                            service,
                            processes,
                            childs,
                            event_tx,
                            log_engine,
                        ) => {
                            cloned_graph
                                .add_vertex(&service, processes, childs, event_tx, log_engine);
                        }
                        GraphCommand::AddEdge(from, to) => {
                            cloned_graph.add_edge(from, to);
                        }
                        GraphCommand::StartService(service, build) => {
                            if build {
                                let mut visited = vec![false; cloned_graph.size()];
                                cloned_graph.build_service(&service, &mut visited).await;
                            }
                            cloned_graph.start_service(&service, &mut visited).await;
                        }
                        GraphCommand::StopService(service) => {
                            cloned_graph.stop_service(&service, &mut visited).await;
                        }
                        GraphCommand::BuildService(service) => {
                            cloned_graph.build_service(&service, &mut visited).await;
                        }
                        GraphCommand::StopServices => {
                            cloned_graph.stop_services().await;
                        }
                        GraphCommand::BuildServices => {
                            cloned_graph.build_services().await;
                        }
                        GraphCommand::StartServices(build) => {
                            if build {
                                cloned_graph.build_services().await;
                            }
                            cloned_graph.start_services().await;
                        }
                    }
                }
            });
        });
        graph
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
        log_engine: LogEngine,
    ) -> usize {
        let mut vertex = Vertex::from(service);

        vertex.driver = Box::new(exec::driver::Driver::new(
            self.project.clone(),
            service,
            processes.clone(),
            event_tx.clone(),
            childs.clone(),
            log_engine.clone(),
        ));

        if let Some(r#use) = service.r#use.clone() {
            if r#use
                .clone()
                .into_iter()
                .any(|(driver, _)| driver == "flox")
            {
                vertex.driver = Box::new(flox::driver::Driver::new(
                    self.project.clone(),
                    service,
                    processes.clone(),
                    event_tx.clone(),
                    childs.clone(),
                    log_engine.clone(),
                ));
            }

            if r#use
                .clone()
                .into_iter()
                .any(|(driver, _)| driver == "docker")
            {
                vertex.driver = Box::new(docker::driver::Driver::new(
                    self.project.clone(),
                    self.context.clone(),
                    service,
                    processes.clone(),
                    event_tx.clone(),
                    childs.clone(),
                    log_engine.clone(),
                ));
            }

            if r#use.into_iter().any(|(driver, _)| driver == "nix") {
                vertex.driver = Box::new(nix::driver::Driver::new(
                    self.project.clone(),
                    service,
                    processes,
                    event_tx,
                    childs,
                    log_engine,
                ));
            }
        }
        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push(Edge { from, to });
    }

    pub async fn start_services(&self) {
        let mut visited = vec![false; self.vertices.len()];
        for vertex in self.vertices.clone().into_iter() {
            self.start_service(&vertex.into(), &mut visited).await;
        }
    }

    #[async_recursion(?Send)]
    pub async fn start_service(&self, service: &Service, visited: &mut Vec<bool>) {
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
            )
            .await;
        }

        println!("Starting service {}", self.vertices[index].name);
        self.vertices[index]
            .driver
            .start(self.project.clone())
            .await
            .unwrap();
    }

    pub async fn stop_services(&self) {
        let mut visited = vec![false; self.vertices.len()];
        for vertex in self.vertices.clone().into_iter() {
            self.stop_service(&vertex.into(), &mut visited).await;
        }
    }

    #[async_recursion(?Send)]
    pub async fn stop_service(&self, service: &Service, visited: &mut Vec<bool>) {
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
            self.stop_service(&service, visited).await;
        }

        println!("Stopping service {}", self.vertices[index].name);
        self.vertices[index]
            .driver
            .stop(self.project.clone())
            .await
            .unwrap();
    }

    pub async fn build_services(&self) {
        let mut visited = vec![false; self.vertices.len()];
        for vertex in self.vertices.clone().into_iter() {
            self.build_service(&vertex.into(), &mut visited).await;
        }
    }

    #[async_recursion(?Send)]
    pub async fn build_service(&self, service: &Service, visited: &mut Vec<bool>) {
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
            let service = self.vertices[edge.to].clone().into();
            self.build_service(&service, visited).await;
        }

        println!("Building service {}", self.vertices[index].name);
        self.vertices[index]
            .driver
            .build(self.project.clone())
            .await
            .unwrap();
    }
}
