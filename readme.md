<h2>To Run the generated .bin in Qemu emulator.</h2>
<i><span>**RUN:** </span>cargo bootimage<i>
<br>
<i><span>**RUN:** </span>qemu-system-x86_64.exe -drive format=raw,file=target\llvm_target_descp\debug\bootimage-OS-With-Rust.bin</i>
