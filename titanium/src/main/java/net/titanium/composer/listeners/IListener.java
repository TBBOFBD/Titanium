package net.titanium.composer.listeners;

public interface IListener {
    void call(Object event);
    Class<?> getTarget();
    int getPriority();
    boolean isStatic();
}
