package io.github.iiiii7d.zylang;

import java.util.ArrayList;
import java.util.stream.Collectors;

enum TokenType {
    COMMENT_START,
    COMMENT_END,
    MULTILINE_COMMENT_START,
    MULTILINE_COMMENT_END,
    COMMENT
}

class Token {
    public String value;
    public TokenType type;
    public int lineNo;
    public int columnNo;

    public Token(String value, TokenType type, int lineNo, int columnNo) {
        this.value = value;
        this.type = type;
        this.lineNo = lineNo;
        this.columnNo = columnNo;
    }
}

public class Lexer {
    private static String charArrayListToString(ArrayList<Character> in) {
        return in.stream().map(String::valueOf).collect(Collectors.joining());
    }

    public static ArrayList<Token> lex(String in) {
        ArrayList<Token> out = new ArrayList<>();
        ArrayList<Character> stack = new ArrayList<>();

        TokenType lastTokenType = null;
        int lineNo, columnNo = 1;
        int tokenLineNo, tokenColumnNo = 1;
        for (char c : in.toCharArray()) {
            if (c == '\n') ++columnNo; lineNo = 0;

            String stackString = charArrayListToString(stack);
            if (lastTokenType == TokenType.COMMENT_START && c == '\n') {
                out.add(new Token(stackString, TokenType.COMMENT, tokenLineNo, tokenColumnNo));
                out.add(new Token("\n", TokenType.COMMENT_END, lineNo, columnNo));
                stack.clear();
                tokenLineNo = lineNo; tokenColumnNo = columnNo;
            } else if (lastTokenType == TokenType.MULTILINE_COMMENT_START && stackString.endsWith("*/")) {
                out.add(new Token(stackString, TokenType.COMMENT, tokenLineNo, tokenColumnNo));
                out.add(new Token("\n", TokenType.MULTILINE_COMMENT_END, lineNo, columnNo));
                stack.clear();
                tokenLineNo = lineNo; tokenColumnNo = columnNo;
            }

            stack.add(c);

            stackString = charArrayListToString(stack);
            switch (stackString.strip()) {
                case "//" -> {
                    out.add(new Token(stackString, lastTokenType = TokenType.COMMENT_START, tokenLineNo, tokenColumnNo));
                    stack.clear();
                    tokenLineNo = lineNo; tokenColumnNo = columnNo;
                }
                case "/*" -> {
                    out.add(new Token(stackString, lastTokenType = TokenType.MULTILINE_COMMENT_START, tokenLineNo, tokenColumnNo));
                    stack.clear();
                    tokenLineNo = lineNo; tokenColumnNo = columnNo;
                }
            }

            ++lineNo;
        }
        return out;
    }
}
