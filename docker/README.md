# This script is to bulid rust-verification-tool(klee only) image. 
Test passed on ubuntu18.04, docker19.03, 2020,10,1. 
The total size of 3 built imgages is about 30GB. 
### build ubuntu:expect 
For some scripts need to be interactive，so we need a ubuntu image with expect.  
The image can be built as follows:(On my desktop, docker need `sudo` privilege) ` 
+ `sudo docker pull ubuntu:18.04`  
+ `sudo docker exec -it ubuntu:18.04`  
+ execute `apt update && apt upgrade && apt install expect -y` in this container 
+ choose district and time zone according to your own situation.(This is why I didn't add this step to dockerfile)
+ execute `exit` in container to exit 
+ `sudo docker ps -a` to see the CONTAINER_ID just modified 
+ then use the CONTAINER_ID `sudo docker commit CONTAINER_ID ubuntu:expect` 
### build rust_verification 
after build ubuntu:expect, `./build` will automatically build this container
### run rust_verification 
`./run` will start the container  
in container,execute: 
+ `cd $HOME/rust-verification-tools/try_klee` 
+ `./cargo_build` (build the crate) 
+ `./klee_test` (test the .bc file with klee)
