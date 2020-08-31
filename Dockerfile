FROM rust:1.46-slim-buster

COPY ./scripts/apt_dependencies.sh /install_dependencies.sh
COPY ./.entrypoint.sh /entrypoint.sh
RUN bash /install_dependencies.sh

ENTRYPOINT ["/entrypoint.sh"]
