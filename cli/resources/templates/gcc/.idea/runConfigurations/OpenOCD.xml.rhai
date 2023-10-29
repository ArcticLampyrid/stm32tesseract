`<component name="ProjectRunConfigurationManager">
  <configuration default="false" name="OCD ${info.project_name}" type="com.jetbrains.cidr.embedded.openocd.conf.type" factoryName="com.jetbrains.cidr.embedded.openocd.conf.factory" REDIRECT_INPUT="false" ELEVATE="false" USE_EXTERNAL_CONSOLE="false" PASS_PARENT_ENVS_2="true" PROJECT_NAME="${info.project_name}" TARGET_NAME="${info.project_name}.elf" version="1" RUN_TARGET_PROJECT_NAME="${info.project_name}" RUN_TARGET_NAME="${info.project_name}.elf">
    <openocd version="1" gdb-port="3333" telnet-port="4444" board-config="$PROJECT_DIR$/openocd.cfg" reset-type="HALT" download-type="UPDATED_ONLY">
      <debugger kind="GDB" isBundled="true" />
    </openocd>
    <method v="2">
      <option name="CLION.COMPOUND.BUILD" enabled="true" />
    </method>
  </configuration>
</component>`