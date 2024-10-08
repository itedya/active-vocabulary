watch-windows:
    docker-compose up -d
    just only-watch

watch-linux:
    sudo docker-compose up -d
    just only-watch

only-watch:
    cargo watch -x run -i target