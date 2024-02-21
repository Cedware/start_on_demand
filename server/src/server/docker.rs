use bollard::container::StartContainerOptions;
use bollard::Docker;

fn connect_to_docker() -> Result<Docker, bollard::errors::Error> {
    Docker::connect_with_socket_defaults()
}

pub async fn stop_container(container_name: &str) -> Result<(), bollard::errors::Error> {
    let docker = connect_to_docker()?;
    docker.stop_container(container_name, None).await?;
    Ok(())
}

pub async fn start_container(container_name: &str) -> Result<(), bollard::errors::Error> {
    let docker = connect_to_docker()?;
    docker.start_container(container_name, None::<StartContainerOptions<String>>).await?;
    Ok(())
}