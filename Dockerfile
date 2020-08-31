FROM rust:1.46-slim-buster

COPY ./scripts/* /
RUN bash /apt_install_dependencies.sh

ENTRYPOINT ["/docker_entrypoint.sh"]
