package net.titanium.corelog.loggers;

import static net.titanium.corelog.ConsoleUtils.*;
import ch.qos.logback.classic.spi.ILoggingEvent;
import ch.qos.logback.core.CoreConstants;
import ch.qos.logback.classic.Level;
import java.text.SimpleDateFormat;

public class ColoredConsoleLogger extends ConsoleLogger {
    @Override
    public String doLayout(ILoggingEvent event) {
        StackTraceElement[] data = event.getCallerData();
        return COLORS.CYAN +
            (
                (new SimpleDateFormat("dd/MM/yyyy-HH:mm:ss"))
                .format(event.getTimeStamp())
            ) +
            " " +
            levelParser(
                event.getLevel()
            ) +
            from8Bit(5) +
            " [" +
            prettyPrintStackElement(data[0]) +
            "] " +
            RESET +
            from8Bit(123) +
            event.getFormattedMessage() +
            RESET +
            CoreConstants.LINE_SEPARATOR
        ;
    }
    @Override
    protected String prettyPrintStackElement(StackTraceElement element) {
        return element.getClassName() +
            "." +
            element.getMethodName() +
            ":" +
            element.getLineNumber()
        ;
    }
    @Override
    protected String levelParser(Level level) {
        if(level.toInt() == Level.TRACE_INT) return MODIFIERS.BOLD + CORELOG.TRACE_WHITE + "TRACE";
        if(level.toInt() == Level.DEBUG_INT) return MODIFIERS.BOLD + CORELOG.DEBUG_GREEN + "DEBUG";
        if(level.toInt() == Level.INFO_INT)  return MODIFIERS.BOLD + CORELOG.INFO_BLUE +   "INFO ";
        if(level.toInt() == Level.WARN_INT)  return MODIFIERS.BOLD + CORELOG.WARN_ORANGE + "WARN ";
        if(level.toInt() == Level.ERROR_INT) return MODIFIERS.BOLD + CORELOG.ERROR_RED +   "ERROR";
        return "NONE ";
    }
}
