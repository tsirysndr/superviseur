pub mod cmd;
pub mod server;
pub mod superviseur;
pub mod types;

pub mod api {
    #[path = ""]
    pub mod superviseur {
        #[path = "superviseur.v1alpha1.rs"]
        pub mod v1alpha1;
    }
    #[path = ""]
    pub mod objects {
        #[path = "objects.v1alpha1.rs"]
        pub mod v1alpha1;
    }
}
