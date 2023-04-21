package net.titanium.composer;

import net.titanium.$;
import net.titanium.composer.listeners.LambdaListener;
import net.titanium.composer.core.DefaultEventBus;

import java.lang.invoke.MethodHandles;
import java.util.*;

public class Composer {
    public static IEventBus getDefault(String packageName) {
        IEventBus bus = new DefaultEventBus();
        bus.registerLambdaFactory(packageName, defaultLambdaFactory);
        return bus;
    }
    public static IEventBus getDefaultRaw() {
        return new DefaultEventBus();
    }
    public static void subscribe(IEventBus bus, Class<?> klazz) {
        registerBusLambdaFactory(bus, klazz);
        bus.subscribe(klazz);
    }
    public static void subscribe(IEventBus bus, Object object) {
        registerBusLambdaFactory(bus, object.getClass());
        bus.subscribe(object);
    }
    private static void registerBusLambdaFactory(IEventBus bus, Class<?> klazz) {
        String pkg = klazz.getName();
        $.LOG.trace("Settings up '"+pkg+"' to be registered to bus");
        String[] split = "\\.".split(pkg);
        ArrayList<String> list = new ArrayList<>();
        Collections.addAll(list, split);
        list.remove(list.size() - 1);
        String name = String.join(".",list);
        $.LOG.trace("Registering Bus Lambda Factory for '"+name+"'");
        bus.registerLambdaFactory(name, defaultLambdaFactory);
    }
    public static <I extends IEventBus> IEventBus getFromClass(Class<I> klazz, String packageName) {
        try {
            I clazz = klazz.getDeclaredConstructor().newInstance();
            clazz.registerLambdaFactory(packageName, defaultLambdaFactory);
            return clazz;
        } catch(Exception e) {
            return getDefault(packageName);
        }
    }
    public static final LambdaListener.LambdaFactory defaultLambdaFactory = (lookupInMethod, klass) ->
        (MethodHandles.Lookup) lookupInMethod.invoke(null, klass, MethodHandles.lookup());
}
