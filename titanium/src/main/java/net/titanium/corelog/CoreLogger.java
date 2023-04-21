package net.titanium.corelog;

import org.slf4j.event.*;
import org.slf4j.*;

@SuppressWarnings("all")
public class CoreLogger {
    public static Logger getLogger(String name) {
        return getLogger(name, Level.DEBUG);
    }
    public static Logger getLogger(Class<?> klazz) {
        return getLogger(klazz, Level.DEBUG);
    }
    public static Logger getLogger(String name, Level level) {
        return getLoggerWithLevel(name, level);
    }
    public static Logger getLogger(Class<?> klazz, Level level) {
        return getLoggerWithLevel(klazz.getSimpleName(), level);
    }
    private static Logger getLoggerWithLevel(String name, Level level) {
        Logger logger = LoggerFactory.getLogger(name);
        logger.atLevel(level);
        return logger;
    }
}
