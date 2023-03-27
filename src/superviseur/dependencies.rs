use crate::types::configuration::Service;

struct Vertex {
    name: String,
}

impl From<&Service> for Vertex {
    fn from(service: Service) -> Self {
        Self { name: service.name }
    }
}

impl Into<Service> for Vertex {
    fn into(self) -> Service {
        Service {
            name: self.name,
            ..Default::default()
        }
    }
}

struct Edge {
    from: usize,
    to: usize,
}

pub struct DependencyGraph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, service: &Service) -> usize {
        let vertex = Vertex::from(service);
        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push(Edge { from, to });
    }

    pub fn start_services(&self) {
        let mut visited = vec![false; self.vertices.len()];
        for vertex in self.vertices.into_iter() {
            self.start_service(vertex.into(), &mut visited);
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

        println!("Starting service {}", service.name);
    }
}
