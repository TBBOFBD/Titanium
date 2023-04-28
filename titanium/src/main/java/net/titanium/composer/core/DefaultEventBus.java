package net.titanium.composer.core;

import net.titanium.$;
import net.titanium.composer.listeners.*;
import net.titanium.composer.*;

import java.util.function.Function;
import java.lang.reflect.Method;
import java.util.concurrent.*;
import java.util.*;

public class DefaultEventBus implements IEventBus {
    private static final class LambdaFactoryInfo {
        public final LambdaListener.LambdaFactory factory;
        public final String packagePrefix;
        public LambdaFactoryInfo(String packagePrefix, LambdaListener.LambdaFactory factory) {
            this.packagePrefix = packagePrefix;
            this.factory = factory;
        }
    }
//    private record LambdaFactoryInfo(String packagePrefix, LambdaListener.LambdaFactory factory) { }
    private final Map<Object, List<IListener>> listenerCache = new ConcurrentHashMap<>();
    private final Map<Class<?>, List<IListener>> staticListenerCache = new ConcurrentHashMap<>();
    private final Map<Class<?>, List<IListener>> listenerMap = new ConcurrentHashMap<>();
    private final List<LambdaFactoryInfo> lambdaFactoryInfos = new ArrayList<>();
    @Override
    public void registerLambdaFactory(String packagePrefix, LambdaListener.LambdaFactory factory) {
        synchronized(lambdaFactoryInfos) {
            lambdaFactoryInfos.add(new LambdaFactoryInfo(packagePrefix, factory));
            $.LOG.debug("Registered new lambda listener for package '" + packagePrefix + "'");
        }
    }
    @Override
    public <T> T post(T event) {
        if(event == null){
            $.LOG.trace("Event was null");
            return null;
        } else {
            $.LOG.trace("Event '" + event.getClass().getSimpleName() + "' created");
        }
        List<IListener> listeners = listenerMap.get(event.getClass());
        if(listeners != null) {
            for(IListener listener : listeners) listener.call(event);
        }
        return event;
    }
    @Override
    public <T extends ICancellable> T post(T event) {
        if(event == null){
            $.LOG.trace("Event was null");
            return null;
        } else {
            $.LOG.trace("Event '" + event.getClass().getSimpleName() + "' created");
        }
        List<IListener> listeners = listenerMap.get(event.getClass());
        if(listeners != null) {
            event.setCancelled(false);
            for(IListener listener : listeners) {
                listener.call(event);
                if(event.isCancelled()) break;
            }
        }
        return event;
    }
    @Override
    public <T extends IOnceCallable> T post(T event) {
        if(event == null){
            $.LOG.trace("Event was null");
            return null;
        }
        if(!event.isFirstCall()) {
            $.LOG.error("Event '" + event.getClass().getSimpleName() + "' was created more than once");
            return null;
        }
        $.LOG.trace("Once Callable Event '" + event.getClass().getSimpleName() + "' created");
        event.setAsCalled();
        List<IListener> listeners = listenerMap.get(event.getClass());
        if(listeners != null) {
            for(IListener listener : listeners) {
                listener.call(event);
            }
        }
        return event;
    }
    @Override
    public void subscribe(Object object) {
        subscribe(getListeners(object.getClass(), object), false);
    }
    @Override
    public void subscribe(Class<?> klass) {
        subscribe(getListeners(klass, null), true);
    }
    @Override
    public void subscribe(IListener listener) {
        subscribe(listener, false);
    }
    private void subscribe(List<IListener> listeners, boolean onlyStatic) {
        for(IListener listener : listeners) subscribe(listener, onlyStatic);
    }
    private void subscribe(IListener listener, boolean onlyStatic) {
        $.LOG.trace("Subscribed to '" + listener.getTarget().getSimpleName() + "', only statics: " + onlyStatic);
        if(onlyStatic && !listener.isStatic()) return;
        insert(listenerMap.computeIfAbsent(listener.getTarget(), aClass -> new CopyOnWriteArrayList<>()), listener);
    }
    private void insert(List<IListener> listeners, IListener listener) {
        int i = 0;
        for(; i < listeners.size(); i++) {
            if(listener.getPriority() > listeners.get(i).getPriority()) break;
        }
        listeners.add(i, listener);
    }
    @Override
    public void unsubscribe(Object object) {
        unsubscribe(getListeners(object.getClass(), object), false);
    }
    @Override
    public void unsubscribe(Class<?> klass) {
        unsubscribe(getListeners(klass, null), true);
    }
    @Override
    public void unsubscribe(IListener listener) {
        unsubscribe(listener, false);
    }
    private void unsubscribe(List<IListener> listeners, boolean staticOnly) {
        for(IListener listener : listeners) unsubscribe(listener, staticOnly);
    }
    private void unsubscribe(IListener listener, boolean staticOnly) {
        List<IListener> l = listenerMap.get(listener.getTarget());
        if(l != null) {
            if(staticOnly && !listener.isStatic()) return;
            l.remove(listener);
        }
    }
    private List<IListener> getListeners(Class<?> klass, Object object) {
        Function<Object, List<IListener>> func = o -> {
            List<IListener> listeners = new CopyOnWriteArrayList<>();
            getListeners(listeners, klass, object);
            return listeners;
        };
        if(object == null) return staticListenerCache.computeIfAbsent(klass, func);
        for(Object key : listenerCache.keySet()) {
            if(key == object) return listenerCache.get(object);
        }
        List<IListener> listeners = func.apply(object);
        listenerCache.put(object, listeners);
        return listeners;
    }
    private void getListeners(List<IListener> listeners, Class<?> klass, Object object) {
        for(Method method : klass.getDeclaredMethods()) {
            if(isValid(method)) {
                listeners.add(new LambdaListener(getLambdaFactory(klass), klass, object, method));
            }
        }
        if(klass.getSuperclass() != null) getListeners(listeners, klass.getSuperclass(), object);
    }
    private boolean isValid(Method method) {
        if(!method.isAnnotationPresent(EventHandler.class)) return false;
        if(method.getReturnType() != void.class) return false;
        if(method.getParameterCount() != 1) return false;
        return !method.getParameters()[0].getType().isPrimitive();
    }
    private LambdaListener.LambdaFactory getLambdaFactory(Class<?> klass) {
        synchronized(lambdaFactoryInfos) {
            for(LambdaFactoryInfo info : lambdaFactoryInfos) {
                if(klass.getName().startsWith(info.packagePrefix)) return info.factory;
            }
        }
        throw new NoLambdaFactoryRegisteredException(klass);
    }
}
