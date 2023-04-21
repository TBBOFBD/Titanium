package net.titanium.events;

import net.titanium.composer.*;

public class InitEvent implements IOnceCallable {
    private boolean firstTime = true;
    private static final InitEvent INSTANCE = new InitEvent();

    public static InitEvent get() {
        return INSTANCE;
    }
    @Override
    public boolean isFirstCall() {
        return firstTime;
    }
    @Override
    public void setAsCalled() {
        firstTime = false;
    }
}
