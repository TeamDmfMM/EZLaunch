use project::dependency::InstanceDep;

#[derive(Serialize, Deserialize)]
pub struct Project {

    deps: Vec<InstanceDep>,
    name: String,
    version: String, //todo versioning

    launcher_profile_name: String

}