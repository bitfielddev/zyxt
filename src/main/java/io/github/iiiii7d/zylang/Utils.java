package io.github.iiiii7d.zylang;

import org.jetbrains.annotations.Contract;
import org.jetbrains.annotations.NotNull;

class Ansi {
    public static final String RESET = "\033[0m";

    public static final String BLACK = "\033[0;30m";
    public static final String RED = "\033[0;31m";
    public static final String GREEN = "\033[0;32m";
    public static final String YELLOW = "\033[0;33m";
    public static final String BLUE = "\033[0;34m";
    public static final String PURPLE = "\033[0;35m";
    public static final String CYAN = "\033[0;36m";
    public static final String WHITE = "\033[0;37m";

    @Contract(pure = true)
    public static @NotNull String black(String s) {
        return BLACK+s+RESET;
    }
    @Contract(pure = true)
    public static @NotNull String red(String s) {
        return RED+s+RESET;
    }
    @Contract(pure = true)
    public static @NotNull String green(String s) {
        return GREEN+s+RESET;
    }
    @Contract(pure = true)
    public static @NotNull String yellow(String s) {
        return YELLOW+s+RESET;
    }
    @Contract(pure = true)
    public static @NotNull String blue(String s) {
        return BLUE+s+RESET;
    }
    @Contract(pure = true)
    public static @NotNull String purple(String s) {
        return PURPLE+s+RESET;
    }
    @Contract(pure = true)
    public static @NotNull String cyan(String s) {
        return CYAN+s+RESET;
    }
    @Contract(pure = true)
    public static @NotNull String white(String s) {
        return WHITE+s+RESET;
    }
}