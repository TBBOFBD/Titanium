package net.titanium.corelog;

/**
 * More Info Here: <a href="https://en.wikipedia.org/wiki/ANSI_escape_code">ANSI Codes</a>
 */
@SuppressWarnings("all")
public class ConsoleUtils {
    public static final class CORELOG {
        public static final String DEBUG_GREEN = from8Bit(36);
        public static final String TRACE_WHITE = from8Bit(195);
        public static final String INFO_BLUE = from8Bit(33);
        public static final String WARN_ORANGE = from8Bit(202);
        public static final String ERROR_RED = from8Bit(196);
    }
    public static final class MODIFIERS {
        public static final String RESET = from4Bit(0);
        public static final String BOLD = from4Bit(1);
        public static final String WEAK = from4Bit(2);
        public static final String ITALIC = from4Bit(3);
        public static final String UNDERLINE = from4Bit(4);
        public static final String SLOW_BLINK = from4Bit(5);
        public static final String RAPID_BLINK = from4Bit(6);
        public static final String INVERT = from4Bit(7);
        public static final String HIDE = from4Bit(8);
        public static final String CROSSED_OUT = from4Bit(9);
        public static final String DEFAULT_FONT = from4Bit(10);
        public static String CUSTOM_FONT(int id) {
            if(id < 11 || id > 19) return DEFAULT_FONT;
            return from4Bit(id);
        }
    }
    public static final class COLORS {
        public static final String BLACK = from4Bit(30);
        public static final String RED = from4Bit(31);
        public static final String GREEN = from4Bit(32);
        public static final String YELLOW = from4Bit(33);
        public static final String BLUE = from4Bit(34);
        public static final String PURPLE = from4Bit(35);
        public static final String CYAN = from4Bit(36);
        public static final String WHITE = from4Bit(37);
    }
    public static final String RESET = from4Bit(0);
    public static String from4Bit(int id) {
        return"\033["+id+"m";
    }
    public static String from8Bit(int id) {
        return from8Bit(id, false);
    }
    public static String from8Bit(int id, boolean background) {
        return"\033["+(background?"48":"38")+";5;"+id+"m";
    }
    public static String from24Bit(int r, int g, int b) {
        return from24Bit(r,g,b,false);
    }
    public static String from24Bit(int r, int g, int b, boolean background) {
        return"\033["+(background?"48":"38")+";2;"+r+";"+g+";"+b+"m";
    }
}
