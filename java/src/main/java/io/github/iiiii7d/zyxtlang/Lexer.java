package io.github.iiiii7d.zyxtlang;

import org.jetbrains.annotations.NotNull;

import java.util.ArrayList;
import java.util.stream.Collectors;

enum TokenType {
    COMMENT_START, // //
    COMMENT_END, // \n
    MULTILINE_COMMENT_START, // /*
    MULTILINE_COMMENT_END, // */
    FLAG, // :
    FLAG_LABEL, // the stuff after :
    UNARY_OPR, // b~, ++, ! etc
    ASSIGNMENT_OPR, // =, +=, etc
    ARITHMETIC_BITWISE_OPR, // +, -, /f, rt, b& etc
    RELATIONAL_OPR, // ==, >, is etc
    LOGICAL_OPR, // &&, ||, ^^ etc
    CONCAT_OPR, // ..
    SWAP_OPR, // ><
    TYPE_OPR, // istype, isnttype et
    LITERAL, // "abc", 3, true, null, etc
    STATEMENT_END, // ;
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
    private static String charArrayListToString(@NotNull ArrayList<Character> in) {
        return in.stream().map(String::valueOf).collect(Collectors.joining());
    }

    public static @NotNull ArrayList<Token> lex(String in) {
        ArrayList<Token> out = new ArrayList<>();
        ArrayList<Character> stack = new ArrayList<>();

        TokenType lastTokenTypeBeforeComment = null;
        TokenType lastTokenType = TokenType.STATEMENT_END;
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
                lastTokenType = lastTokenTypeBeforeComment;
                lastTokenTypeBeforeComment = null;
                out.add(new Token(stackString.substring(0, stackString.length()-1), TokenType.COMMENT, tokenLineNo, tokenColumnNo));
                out.add(new Token("\n", TokenType.COMMENT_END, lineNo-1, prevColumnNo));
                stack.clear();
                ignoreWhitespace = true;
                tokenLineNo = lineNo;
                tokenColumnNo = columnNo;
            } else if (lastTokenType == TokenType.MULTILINE_COMMENT_START && stackString.endsWith("*/")) { //multiline comment ending
                lastTokenType = lastTokenTypeBeforeComment;
                lastTokenTypeBeforeComment = null;
                out.add(new Token(stackString.substring(0, stackString.length()-2), TokenType.COMMENT, tokenLineNo, tokenColumnNo));
                out.add(new Token("*/", TokenType.MULTILINE_COMMENT_END, lineNo, columnNo-2));
                stack.clear();
                ignoreWhitespace = true;
                tokenLineNo = lineNo;
                tokenColumnNo = columnNo;
            } else if ((lastTokenType == TokenType.STATEMENT_END || lastTokenType == TokenType.FLAG) && stackString.endsWith(":")) { // flag end&start
                if (lastTokenType == TokenType.FLAG)
                    out.add(new Token(stackString.substring(0, stackString.length()-1), TokenType.FLAG_LABEL, tokenLineNo, tokenColumnNo));
                out.add(new Token(stackString, lastTokenType = TokenType.FLAG, lineNo, tokenColumnNo-1));
                stack.clear();
                tokenLineNo = lineNo;
                tokenColumnNo = columnNo;
            }

            stackString = charArrayListToString(stack).strip();
            if (Character.isWhitespace(c)) {
                if (lastTokenType == TokenType.FLAG) { // flag end
                    out.add(new Token(stackString, TokenType.FLAG_LABEL, tokenLineNo, tokenColumnNo));
                    stack.clear();
                    tokenLineNo = lineNo;
                    tokenColumnNo = columnNo;
                }

                TokenType token = null;
                switch (stackString) {
                    case "=", "+=", "-=", "*=", "/=", "/f=", "/c=", "/~=", "%=", "b&=", "b|=", "b^=", "b<<=", "b>>=", "b>>>=" -> token = TokenType.ASSIGNMENT_OPR;
                    case "+", "-", "*", "/", "/f", "/c", "/~", "%", "rt", "lg", "b&", "b|", "b^", "b<<", "b>>", "b>>>" -> token = TokenType.ARITHMETIC_BITWISE_OPR;
                    case "==", ">", "<", ">=", "<=", "!=", "===", "!==", "is", "isnt" -> token = TokenType.RELATIONAL_OPR;
                    case "&&", "||", "^^" -> token = TokenType.LOGICAL_OPR;
                    case ".." -> token = TokenType.CONCAT_OPR;
                    case "><" -> token = TokenType.SWAP_OPR;
                    case "istype", "isnttype" -> token = TokenType.TYPE_OPR;
                    case "null", "true", "false" -> token = TokenType.LITERAL;
                }
                if (token != null) {
                    out.add(new Token(stackString, token, tokenLineNo, tokenColumnNo));
                    stack.clear();
                    tokenLineNo = lineNo;
                    tokenColumnNo = columnNo;
                }
            }

            switch (stackString) {
                case "//" -> { // comment starting
                    lastTokenTypeBeforeComment = lastTokenType;
                    out.add(new Token(stackString, lastTokenType = TokenType.COMMENT_START, tokenLineNo, tokenColumnNo));
                    stack.clear();
                    ignoreWhitespace = false;
                    tokenLineNo = lineNo;
                    tokenColumnNo = columnNo;
                }
                case "/*" -> { // multiline comment ending
                    lastTokenTypeBeforeComment = lastTokenType;
                    out.add(new Token(stackString, lastTokenType = TokenType.MULTILINE_COMMENT_START, tokenLineNo, tokenColumnNo));
                    stack.clear();
                    ignoreWhitespace = false;
                    tokenLineNo = lineNo;
                    tokenColumnNo = columnNo;
                }
                default -> {
                    if (ignoreWhitespace && Character.isWhitespace(c) && stackString.strip().length() != 0) { // variable
                        out.add(new Token(stackString, lastTokenType = TokenType.VARIABLE, tokenLineNo, tokenColumnNo));
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
