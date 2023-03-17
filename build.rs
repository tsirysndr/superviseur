fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().out_dir("src/api").compile(
        &[
            "proto/objects/v1alpha1/service.proto",
            "proto/superviseur/v1alpha1/control.proto",
            "proto/superviseur/v1alpha1/core.proto",
            "proto/superviseur/v1alpha1/logging.proto",
        ],
        &["proto"],
    )?;
    Ok(())
}
