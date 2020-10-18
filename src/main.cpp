#include <iostream>
#include "commandline.h"

int main(int arg_count, char *args[]) {
    try {
        return command_line::parse(arg_count, args);
    } catch (const std::exception& e) {
        std::cout << e.what() << std::endl;

        return 1;
    }
}
