FROM rust:1.44-slim-buster

COPY ./infra/scripts/dependencies.sh /install_dependencies.sh
COPY ./.entrypoint.sh /entrypoint.sh
RUN bash /install_dependencies.sh

ENTRYPOINT ["/entrypoint.sh"]
