FROM archlinux:base-devel AS base

RUN pacman -Syu --noconfirm

# Install dependancies needed by all steps including runtime step
RUN pacman -S --noconfirm --needed ffmpeg vapoursynth ffms2 libvpx mkvtoolnix-cli svt-av1 vapoursynth-plugin-lsmashsource vmaf tesseract-data-eng

FROM base AS build-base

# Install dependancies needed by build steps
RUN pacman -S --noconfirm --needed rust clang nasm git

RUN cargo install cargo-chef
WORKDIR /tmp/av1an-vmaf


FROM build-base AS planner

COPY . .
RUN cargo chef prepare


FROM build-base AS build

COPY --from=planner /tmp/av1an-vmaf/recipe.json recipe.json
RUN cargo chef cook --release

# Build av1an
COPY . /tmp/av1an-vmaf

RUN cargo build --release && \
    mv ./target/release/av1an-vmaf /usr/local/bin && \
    cd .. && rm -rf ./av1an-vmaf

FROM base AS runtime

# Create user
RUN useradd -ms /bin/bash app_user
USER app_user

COPY --from=build /usr/local/bin/av1an-vmaf /usr/local/bin/av1an-vmaf

ENTRYPOINT [ "/usr/local/bin/av1an-vmaf" ]
