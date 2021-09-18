package io.github.iiiii7d.zylang;

import java.util.ArrayList;
import java.util.stream.Collectors;

enum TokenType {
    COMMENT_START,
    COMMENT_END,
    MULTILINE_COMMENT_START,
    MULTILINE_COMMENT_END,
    COMMENT,
    VARIABLE
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

    @Override
    public String toString() {
        return ("Token{" +
                "value='" + value.replace("\n", "\\n").replace("\r", "\\r") + '\'' +
                ", type=" + type +
                ", lineNo=" + lineNo +
                ", columnNo=" + columnNo +
                '}');
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
        boolean ignoreWhitespace = true;
        int lineNo = 1, columnNo = 1;
        int tokenLineNo = 1, tokenColumnNo = 1;
        int prevColumnNo = 0;

        for (char c : (in + "\n").toCharArray()) {
            if (c == '\n') { // if newline, update lineNo
                lineNo++;
                prevColumnNo = columnNo;
                columnNo = 1;
            } else columnNo++;
            if (c == '\r' && ignoreWhitespace) continue;

            stack.add(c);

            String stackString = charArrayListToString(stack);
            if (lastTokenType == TokenType.COMMENT_START && c == '\n') { // comment ending
                out.add(new Token(stackString.substring(0, stackString.length()-1), TokenType.COMMENT, tokenLineNo, tokenColumnNo));
                out.add(new Token("\n", lastTokenType = TokenType.COMMENT_END, lineNo-1, prevColumnNo));
                stack.clear();
                ignoreWhitespace = true;
                tokenLineNo = lineNo;
                tokenColumnNo = columnNo;
            } else if (lastTokenType == TokenType.MULTILINE_COMMENT_START && stackString.endsWith("*/")) { //multiline comment ending
                out.add(new Token(stackString.substring(0, stackString.length()-2), TokenType.COMMENT, tokenLineNo, tokenColumnNo));
                out.add(new Token("*/", lastTokenType = TokenType.MULTILINE_COMMENT_END, lineNo, columnNo-2));
                stack.clear();
                ignoreWhitespace = true;
                tokenLineNo = lineNo;
                tokenColumnNo = columnNo;
            }

            stackString = charArrayListToString(stack);
            switch (stackString.strip()) {
                case "" -> { // already handled up there
                    if (ignoreWhitespace && Character.isWhitespace(c)) { // if it's whitespace, push forward the token coordinates
                        tokenLineNo = lineNo;
                        tokenColumnNo = columnNo;
                    }
                }
                case "//" -> { // comment starting
                    out.add(new Token(stackString.strip(), lastTokenType = TokenType.COMMENT_START, tokenLineNo, tokenColumnNo));
                    stack.clear();
                    ignoreWhitespace = false;
                    tokenLineNo = lineNo;
                    tokenColumnNo = columnNo;
                }
                case "/*" -> { // multiline comment ending
                    out.add(new Token(stackString.strip(), lastTokenType = TokenType.MULTILINE_COMMENT_START, tokenLineNo, tokenColumnNo));
                    stack.clear();
                    ignoreWhitespace = false;
                    tokenLineNo = lineNo;
                    tokenColumnNo = columnNo;
                }
                default -> { // if not it's probably a variable
                    if (ignoreWhitespace && Character.isWhitespace(c) && stackString.strip().length() != 0) {
                        out.add(new Token(stackString.strip(), lastTokenType = TokenType.VARIABLE, tokenLineNo, tokenColumnNo));
                        stack.clear();
                        tokenLineNo = lineNo;
                        tokenColumnNo = columnNo;
                    } else if (ignoreWhitespace && Character.isWhitespace(c)) { // if it's whitespace, push forward the token coordinates
                        tokenLineNo = lineNo;
                        tokenColumnNo = columnNo;
                    }
                }
            }
        }
        return out;
    }
}
