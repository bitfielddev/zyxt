package io.github.iiiii7d.zylang;

import java.util.ArrayList;

public class Lexer {
    public static ArrayList<Character> lex(String in) {
        ArrayList<Character> out = new ArrayList<>();
        for (char c : in.toCharArray()) {
            //if (Character.compare(c, '/') == 0) {}
            out.add(c);
        }
        return out;
    }
}
