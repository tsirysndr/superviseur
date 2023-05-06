use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_project.graphql",
    response_derives = "Debug"
)]
pub struct ProjectQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/list_projects.graphql",
    response_derives = "Debug"
)]
pub struct ProjectsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/list_services.graphql",
    response_derives = "Debug"
)]
pub struct ServicesQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/add_env_variable.graphql",
    response_derives = "Debug"
)]
pub struct CreateEnvVar;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/update_env_variable.graphql",
    response_derives = "Debug"
)]
pub struct UpdateEnvVar;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/remove_env_variable.graphql",
    response_derives = "Debug"
)]
pub struct DeleteEnvVar;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/processes.graphql",
    response_derives = "Debug"
)]
pub struct ProcessesQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/logs.graphql",
    response_derives = "Debug"
)]
pub struct LogsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/start_service.graphql",
    response_derives = "Debug"
)]
pub struct StartService;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/stop_service.graphql",
    response_derives = "Debug"
)]
pub struct StopService;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/restart_service.graphql",
    response_derives = "Debug"
)]
pub struct RestartService;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/status.graphql",
    response_derives = "Debug"
)]
pub struct StatusQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/start_all.graphql",
    response_derives = "Debug"
)]
pub struct StartAllServices;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/stop_all.graphql",
    response_derives = "Debug"
)]
pub struct StopAllServices;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/restart_all.graphql",
    response_derives = "Debug"
)]
pub struct RestartAllServices;
