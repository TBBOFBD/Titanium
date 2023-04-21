package net.titanium.os;

import java.net.*;
import java.io.*;

@SuppressWarnings("all")
public enum OperatingSystem {
    LINUX,
    WINDOWS {
        @Override
        protected String[] URLOpenCommand(URL url) {
            return new String[] { "rundll32", "url.dll,FileProtocolHandler", url.toString() };
        }
    },
    OSX {
        @Override
        protected String[] URLOpenCommand(URL url) {
            return new String[] { "open", url.toString() };
        }
    },
    UNKNOWN;
    public static OperatingSystem getCurrentOS() {
        String os = System.getProperty("os.name").toLowerCase(java.util.Locale.ROOT);
        if (os.contains("linux") || os.contains("unix"))  return OperatingSystem.LINUX;
        if (os.contains("mac")) return OperatingSystem.OSX;
        if (os.contains("win")) return OperatingSystem.WINDOWS;
        return OperatingSystem.UNKNOWN;
    }
    public void open(URL url) throws IOException {
        Runtime.getRuntime().exec(URLOpenCommand(url));
    }
    public void open(String url) throws IOException {
        open(new URL(url));
    }
    public void open(File file) throws IOException {
        open(file.toURI().toURL());
    }
    protected String[] URLOpenCommand(URL url) {
        String string = url.toString();
        if("file".equals(url.getProtocol())) {
            string = string.replace("file:", "file://");
        }
        return new String[] { "xdg-open", string };
    }
}