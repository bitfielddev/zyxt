package io.github.iiiii7d.zylang;

import java.util.Arrays;

import net.sourceforge.argparse4j.ArgumentParsers;

public class Main {
    public static void main(String[] args) {
        System.out.println(Arrays.toString(args));
        ArgumentParser parser = ArgumentParsers.newFor("Checksum").build()
                //.defaultHelp(true)
                .description("Calculate checksum of given files.");
    }
}
