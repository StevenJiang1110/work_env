FROM rust_klee:v3.0

RUN cd $HOME && git clone https://github.com/project-oak/rust-verification-tools.git
RUN cd $HOME/rust-verification-tools && export PATH="$HOME/.cargo/bin:$PATH" && export USER="jjf" && cargo new try_klee
COPY main.rs /root/rust-verification-tools/try_klee/src/
COPY cargo_build /root/rust-verification-tools/try_klee/
COPY cargo_tail /root/rust-verification-tools/try_klee/
COPY klee_test /root/rust-verification-tools/try_klee/
RUN cd $HOME/rust-verification-tools/try_klee && chmod +x cargo_build && chmod +x klee_test && cat cargo_tail >> Cargo.toml
CMD /bin/bash

