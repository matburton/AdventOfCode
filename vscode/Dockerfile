
FROM ubuntu:24.10

RUN ln -s /usr/share/zoneinfo/Europe/London /etc/localtime

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y tzdata \
    && apt install -y wget build-essential

ENV RUSTUP_HOME=/rust/rustup

RUN wget -qO- https://sh.rustup.rs \
  | CARGO_HOME=/rust/cargo sh -s -- -y -c rust-src --no-modify-path
    
ENV PATH="$PATH:/rust/cargo/bin"

RUN --mount=type=cache,target=/downloads \
    wget -O /downloads/vscode.deb \
         --timestamping \
         "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64"

RUN --mount=type=cache,target=/downloads \
    apt install -y /downloads/vscode.deb
    
COPY vscode/settings.json /vscode/data/User/

ENV DONT_PROMPT_WSL_INSTALL=1

RUN code --install-extension rust-lang.rust-analyzer \
    --user-data-dir /vscode/data \
    --extensions-dir /vscode/extensions

RUN --mount=type=cache,target=/downloads \
    wget -O /downloads/codelldb.vsix \
         --timestamping \
         "https://github.com/vadimcn/codelldb/releases/latest/download/codelldb-linux-x64.vsix"
    
RUN --mount=type=cache,target=/downloads \
    code --install-extension /downloads/codelldb.vsix \
         --user-data-dir /vscode/data \
         --extensions-dir /vscode/extensions
    
RUN chmod -R a+rw /vscode/data

ENTRYPOINT ["code", \
            "--no-sandbox", \
            "--user-data-dir", \
            "/vscode/data", \
            "--extensions-dir", \
            "/vscode/extensions", \
            "--wait", \
            "/workspace/2024"]
