FROM ubuntu:expect

RUN apt-get update && apt-get upgrade
RUN apt-get install build-essential libcap-dev git cmake curl libncurses5-dev python-minimal python3-pip unzip libtcmalloc-minimal4 libgoogle-perftools-dev libsqlite3-dev doxygen -y
RUN pip3 install tabulate wllvm lit toml colored termcolor
RUN cd $HOME && git clone https://github.com/rust-lang/rust.git
RUN cd $HOME/rust && git checkout 1.46.0 && cp config.toml.example config.toml && sed -i 's/AArch64;ARM;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86/X86/g' config.toml && RUSTFLAGS_STAGE_NOT_0="-Cembed-bitcode=yes" && ./x.py build
RUN apt-get install expect -y
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf > $HOME/rustup.sh
COPY install_rushup /root/install_rushup
RUN cd $HOME && chmod +x install_rushup && chmod +x rustup.sh && ./install_rushup
RUN echo "export PATH="\$HOME/.cargo/bin:\$PATH"" >> $HOME/.bashrc
RUN export PATH="$HOME/.cargo/bin:$PATH" && cd $HOME/rust && rustup toolchain link stage1 build/x86*/stage1 && rustup default stage1
RUN echo "export USER="jjf"" >> $HOME/.bashrc
