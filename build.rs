fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute("objects.v1alpha1.Project", "#[derive(serde::Serialize)]")
        .type_attribute("objects.v1alpha1.Service", "#[derive(serde::Serialize)]")
        .type_attribute(
            "superviseur.v1alpha1.ListProjectsResponse",
            "#[derive(serde::Serialize)]",
        )
        .type_attribute(
            "superviseur.v1alpha1.GetProjectResponse",
            "#[derive(serde::Serialize)]",
        )
        .out_dir("src/api")
        .compile(
            &[
                "proto/objects/v1alpha1/service.proto",
                "proto/superviseur/v1alpha1/control.proto",
                "proto/superviseur/v1alpha1/core.proto",
                "proto/superviseur/v1alpha1/logging.proto",
                "proto/superviseur/v1alpha1/project.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
