FROM rust:1.46-slim-buster

COPY ./scripts/* /
RUN bash /setup_test_env.sh

ENTRYPOINT ["/docker_entrypoint.sh"]
