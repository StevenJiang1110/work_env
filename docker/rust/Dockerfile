FROM ubuntu:expect

RUN apt-get update && apt-get upgrade
RUN apt-get install build-essential libcap-dev git cmake curl libncurses5-dev python-minimal python3-pip unzip libtcmalloc-minimal4 libgoogle-perftools-dev libsqlite3-dev doxygen -y
RUN pip3 install tabulate wllvm lit toml colored termcolor
RUN cd $HOME && git clone https://github.com/rust-lang/rust.git
RUN cd $HOME/rust && git checkout 1.46.0 && cp config.toml.example config.toml && sed -i 's/AArch64;ARM;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86/X86/g' config.toml && RUSTFLAGS_STAGE_NOT_0="-Cembed-bitcode=yes" && ./x.py build
RUN apt install libssl-dev pkg-config -y
RUN cd $HOME/rust && RUSTFLAGS_STAGE_NOT_0="-Cembed-bitcode=yes" && ./x.py build src/tools/cargo
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf > $HOME/rustup.sh
COPY install_rushup /root/install_rushup
RUN cd $HOME && chmod +x install_rushup && chmod +x rustup.sh && ./install_rushup
RUN echo "export PATH="\$PATH:\$HOME/rust/build/x86_64-unknown-linux-gnu/stage2-tools-bin"" >> $HOME/.bashrc
RUN echo "export PATH="\$PATH:\$HOME/.cargo/bin"" >> $HOME/.bashrc
RUN export PATH="$PATH:$HOME/rust/build/x86_64-unknown-linux-gnu/stage2-tools-bin:$HOME/.cargo/bin" && cd $HOME/rust && rustup toolchain link stage2 build/x86*/stage2 && rustup default stage2
RUN echo "export USER="jjf"" >> $HOME/.bashrc
