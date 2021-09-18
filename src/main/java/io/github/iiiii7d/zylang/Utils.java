package io.github.iiiii7d.zylang;

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

    public static String black(String s) {
        return BLACK+s+RESET;
    }
    public static String red(String s) {
        return RED+s+RESET;
    }
    public static String green(String s) {
        return GREEN+s+RESET;
    }
    public static String yellow(String s) {
        return YELLOW+s+RESET;
    }
    public static String blue(String s) {
        return BLUE+s+RESET;
    }
    public static String purple(String s) {
        return PURPLE+s+RESET;
    }
    public static String cyan(String s) {
        return CYAN+s+RESET;
    }
    public static String white(String s) {
        return WHITE+s+RESET;
    }
}