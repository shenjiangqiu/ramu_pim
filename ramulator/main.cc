#include <DDR4.h>
#include <Main.h>
#include <MemSpecParser.h>
#include <RamulatorConfig.h>
#include <concepts>
#include <fmt/format.h>
#include <ramu_pim_rust.h>
#include <ramulator_wrapper.h>

int main(int argc, const char *argv[]) {
  init_logger();
  auto ramulator = new ramulator_wrapper(
      ramulator::Config("../../ramulator/configs/DDR4-config.cfg"), 64,
      "stat.txt");
  ramulator->send(0x0, false);
  ramulator->send(0x1, false);
  while (!ramulator->return_available()) {
    ramulator->cycle();
  }
  auto ret = ramulator->pop();
  rinfo(fmt::format("ret: {}", ret));

  delete ramulator;
}