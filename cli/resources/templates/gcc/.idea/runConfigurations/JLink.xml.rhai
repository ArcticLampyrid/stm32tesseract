let jlink_device = info.target_mcu;
jlink_device.pop(2);

`<component name="ProjectRunConfigurationManager">
  <configuration default="false" name="JLink" type="com.jetbrains.cidr.embedded.customgdbserver.type" factoryName="com.jetbrains.cidr.embedded.customgdbserver.factory" PROGRAM_PARAMS="-singlerun -nogui -if swd -port 50000 -swoport 50001 -telnetport 50002 -device ${jlink_device}" REDIRECT_INPUT="false" ELEVATE="false" USE_EXTERNAL_CONSOLE="false" PASS_PARENT_ENVS_2="true" PROJECT_NAME="${info.project_name}" TARGET_NAME="${info.project_name}.elf" version="1" RUN_TARGET_PROJECT_NAME="${info.project_name}" RUN_TARGET_NAME="${info.project_name}.elf">
    <custom-gdb-server version="1" gdb-connect="tcp:localhost:50000" executable="C:/Program Files/SEGGER/JLink/JLinkGDBServerCL.exe" warmup-ms="0" download-type="UPDATED_ONLY" reset-cmd="monitor reset" reset-type="AFTER_DOWNLOAD">
      <debugger kind="GDB" isBundled="true" />
    </custom-gdb-server>
    <method v="2">
      <option name="CLION.COMPOUND.BUILD" enabled="true" />
    </method>
  </configuration>
</component>`