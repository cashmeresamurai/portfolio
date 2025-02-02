FROM ghcr.io/railwayapp/nixpacks:ubuntu-1736208272

ENTRYPOINT ["/bin/bash", "-l", "-c"]
WORKDIR /app/


COPY .nixpacks/nixpkgs-ef56e777fedaa4da8c66a150081523c5de1e0171.nix .nixpacks/nixpkgs-ef56e777fedaa4da8c66a150081523c5de1e0171.nix
RUN nix-env -if .nixpacks/nixpkgs-ef56e777fedaa4da8c66a150081523c5de1e0171.nix && nix-collect-garbage -d


ARG NIXPACKS_METADATA ROCKET_ADDRESS
ENV NIXPACKS_METADATA=$NIXPACKS_METADATA ROCKET_ADDRESS=$ROCKET_ADDRESS

# setup phase
# noop

# build phase
COPY . /app/.
RUN --mount=type=cache,id=vIFANgj8vo-/root/cargo/git,target=/root/.cargo/git --mount=type=cache,id=vIFANgj8vo-/root/cargo/registry,target=/root/.cargo/registry --mount=type=cache,id=vIFANgj8vo-target,target=/app/target mkdir -p bin
RUN --mount=type=cache,id=vIFANgj8vo-/root/cargo/git,target=/root/.cargo/git --mount=type=cache,id=vIFANgj8vo-/root/cargo/registry,target=/root/.cargo/registry --mount=type=cache,id=vIFANgj8vo-target,target=/app/target cargo build --release
RUN --mount=type=cache,id=vIFANgj8vo-/root/cargo/git,target=/root/.cargo/git --mount=type=cache,id=vIFANgj8vo-/root/cargo/registry,target=/root/.cargo/registry --mount=type=cache,id=vIFANgj8vo-target,target=/app/target cp target/release/portfolio bin




# start
COPY . /app

CMD ["./bin/portfolio"]

