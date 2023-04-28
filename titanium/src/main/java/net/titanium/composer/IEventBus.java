package net.titanium.composer;

import net.titanium.composer.listeners.*;

public interface IEventBus {
    void registerLambdaFactory(String packagePrefix, LambdaListener.LambdaFactory factory);
    <T> T post(T event);
    <T extends ICancellable> T post(T event);
    <T extends IOnceCallable> T post(T event);
    void subscribe(Object object);
    void subscribe(Class<?> klass);
    void subscribe(IListener listener);
    void unsubscribe(Object object);
    void unsubscribe(Class<?> klass);
    void unsubscribe(IListener listener);
}
