package net.titanium.corelog.loggers;

import ch.qos.logback.classic.Level;
import ch.qos.logback.classic.spi.ILoggingEvent;
import ch.qos.logback.core.CoreConstants;
import ch.qos.logback.core.LayoutBase;

import java.text.SimpleDateFormat;;

public class ConsoleLogger extends LayoutBase<ILoggingEvent> {
    public String doLayout(ILoggingEvent event) {
        StackTraceElement[] data = event.getCallerData();
        return (
                (new SimpleDateFormat("dd/MM/yyyy-HH:mm:ss"))
                .format(event.getTimeStamp())
            ) +
            " " +
            levelParser(
                event.getLevel()
            ) +
            " [" +
            prettyPrintStackElement(data[0]) +
            "] " +
            event.getFormattedMessage() +
            CoreConstants.LINE_SEPARATOR
        ;
    }
    protected String prettyPrintStackElement(StackTraceElement element) {
        return element.getClassName() +
            "." +
            element.getMethodName() +
            ":" +
            element.getLineNumber()
        ;
    }
    protected String levelParser(Level level) {
        if(level.toInt() == Level.TRACE_INT) return "TRACE";
        if(level.toInt() == Level.DEBUG_INT) return "DEBUG";
        if(level.toInt() == Level.INFO_INT)  return "INFO ";
        if(level.toInt() == Level.WARN_INT)  return "WARN ";
        if(level.toInt() == Level.ERROR_INT) return "ERROR";
        return "NONE ";
    }
}
