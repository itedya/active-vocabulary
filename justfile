watch-windows:
    docker-compose up -d
    cargo watch -x run -i target

watch-linux:
    sudo docker-compose up -d
    cargo watch -x run -i target