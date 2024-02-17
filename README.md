Example usage with a palworld server: 
```
services:
    palworld:
        image: thijsvanloef/palworld-server-docker:latest
        restart: unless-stopped
        container_name: palworld-server
        stop_grace_period: 30s # Set to however long you are willing to wait for the container to gracefully stop
        ports:
            - 27015:27015/udp
        environment:
            PUID: 1000
            PGID: 1000
            PORT: 8211 # Optional but recommended
            PLAYERS: 16 # Optional but recommended
            MULTITHREADING: true
            RCON_ENABLED: true
            RCON_PORT: 25575
            TZ: "UTC"
            ADMIN_PASSWORD: "adminPasswordHere"
            COMMUNITY: false  # Enable this if you want your server to show up in the community servers tab, USE WITH SERVER_PASSWORD!
            SERVER_NAME: "World of Pals"
            SERVER_DESCRIPTION: "palworld-server-docker by Thijs van Loef"
        volumes:
            - ./palworld:/palworld/
    start_on_demand:
        image:  ghcr.io/cedware/start_on_demand:3fccc76a64332ca0d4e7fd8d1991789510271878
        restart: unless-stopped
        container_name: start_on_demand
        ports:
            - 8211:8211/udp
        environment:
            RUST_LOG: info
            MODE: udp
            LOCAL_ADDR: 0.0.0.0:8211
            REMOTE_ADDR: palworld-server:8211
            DISCONNECT_TIMEOUT: 60
            STOP_CONTAINER_TIMEOUT: 60
            CONTAINER_NAME: palworld-server
        volumes:
            - /var/run/docker.sock:/var/run/docker.sock
```