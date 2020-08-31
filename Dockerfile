FROM rust:1.46-slim-buster

COPY ./scripts/* /
RUN bash /apt_install_dependencies.sh
RUN bash /setup_database.sh

ENTRYPOINT ["/docker_entrypoint.sh"]
