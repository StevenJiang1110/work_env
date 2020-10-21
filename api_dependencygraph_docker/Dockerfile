FROM rvt_rustc:latest

RUN cd ${RUSTC_DIR} && ./x.py build --incremental --stage 1 src/tools/rustdoc
RUN apt install vim -y
ENV PATH="${RUSTC_DIR}/build/x86_64-unknown-linux-gnu/stage1/bin:${PATH}"
#USER ${USERNAME}
#set root passwd
COPY passwd_root ${USER_HOME}
RUN chmod +x ${USER_HOME}/passwd_root && cd ${USER_HOME} && ./passwd_root && rm ${USER_HOME}/passwd_root
RUN apt update && apt install -y --no-install-recommends openssh-server
RUN sed -i 's/^#PermitRootLogin prohibit-password/PermitRootLogin yes/g' /etc/ssh/sshd_config
RUN mkdir /root/.ssh
COPY authorized_keys /root/.ssh
RUN echo "export PATH="${RUSTC_DIR}/build/x86_64-unknown-linux-gnu/stage1-tools-bin:\$PATH"" >> $HOME/.bashrc
RUN echo "export PATH="${RUSTC_DIR}/build/x86_64-unknown-linux-gnu/llvm/bin:\$PATH"" >> $HOME/.bashrc
RUN echo "export PATH="${RUSTC_DIR}/build/x86_64-unknown-linux-gnu/stage1/bin:${PATH}:\$PATH"" >> $HOME/.bashrc
ARG GIT_USERNAME="jjf"
ARG GIT_EMAIL="stevenjiang1110@gmail.com"
ARG GIT_URL="http://10.176.65.38/jjf/api-dependency-graph.git"
RUN git config --global user.name ${GIT_USERNAME}
RUN git config --global user.email ${GIT_EMAIL}
RUN cd ${RUSTC_DIR} && git remote rename origin old-origin && git remote add origin ${GIT_URL} && git fetch origin
COPY gen_public_key ${USER_HOME}
RUN chmod +x ${USER_HOME}/gen_public_key && cd ${USER_HOME} && ./gen_public_key && rm ${USER_HOME}/gen_public_key
WORKDIR ${USER_HOME}
ENV USER=${USERNAME}
