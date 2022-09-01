#include <iostream>
#include <ramu_pim_rust.h>
int main() {

  init_logger();
  rinfo("Hello, world!");
  rdebug("Hello, world!");
  rinfo_with_target("ramu_pim_rust", "Hello, world!");
}