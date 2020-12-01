FROM rust:1.48-slim-buster

COPY ./scripts/* /
RUN bash /setup_test_env.sh

ENTRYPOINT ["/docker_entrypoint.sh"]
