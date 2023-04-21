package net.titanium.composer.listeners;

import net.titanium.$;
import net.titanium.composer.EventHandler;

import java.util.function.*;
import java.lang.reflect.*;
import java.lang.invoke.*;

public class LambdaListener implements IListener {
    public interface LambdaFactory {
        MethodHandles.Lookup create(Method lookupInMethod, Class<?> klass) throws InvocationTargetException, IllegalAccessException;
    }
    private static boolean isJava1dot8;
    private static Constructor<MethodHandles.Lookup> lookupConstructor;
    private static Method privateLookupInMethod;
    private final Class<?> target;
    private final boolean isStatic;
    private final int priority;
    private Consumer<Object> executor;
    public LambdaListener(LambdaFactory factory, Class<?> klass, Object object, Method method) {
        this.target = method.getParameters()[0].getType();
        this.isStatic = Modifier.isStatic(method.getModifiers());
        this.priority = method.getAnnotation(EventHandler.class).priority();
        try {
            String name = method.getName();
            MethodHandles.Lookup lookup;
            if (isJava1dot8) {
                boolean a = lookupConstructor.canAccess(null);
                lookupConstructor.setAccessible(true);
                lookup = lookupConstructor.newInstance(klass);
                lookupConstructor.setAccessible(a);
            } else {
                lookup = factory.create(privateLookupInMethod, klass);
            }
            MethodType methodType = MethodType.methodType(void.class, method.getParameters()[0].getType());
            MethodHandle methodHandle;
            MethodType invokedType;
            if(isStatic) {
                methodHandle = lookup.findStatic(klass, name, methodType);
                invokedType = MethodType.methodType(Consumer.class);
            } else {
                methodHandle = lookup.findVirtual(klass, name, methodType);
                invokedType = MethodType.methodType(Consumer.class, klass);
            }
            MethodHandle lambdaFactory = LambdaMetafactory.metafactory(
                lookup,
                "accept",
                invokedType,
                MethodType.methodType(void.class, Object.class),
                methodHandle,
                methodType
            ).getTarget();
            if(isStatic){
                this.executor = (Consumer<Object>) lambdaFactory.invoke();
            } else {
                this.executor = (Consumer<Object>) lambdaFactory.invoke(object);
            }
        } catch (Throwable throwable) {
            throwable.printStackTrace();
        }
    }
    @Override
    public void call(Object event) {
        executor.accept(event);
    }
    @Override
    public Class<?> getTarget() {
        return target;
    }
    @Override
    public int getPriority() {
        return priority;
    }
    @Override
    public boolean isStatic() {
        return isStatic;
    }
    static {
        try {
            isJava1dot8 = System.getProperty("java.version").startsWith("1.8");
            if(isJava1dot8) {
                lookupConstructor = MethodHandles.Lookup.class.getDeclaredConstructor(Class.class);
            } else {
                privateLookupInMethod = MethodHandles.class.getDeclaredMethod(
                    "privateLookupIn", Class.class, MethodHandles.Lookup.class);
            }
        } catch(Exception e) {
            $.LOG.error(e.getMessage());
        }
    }
    @Override
    public String toString() {
        return "LambdaListener: { target: " + this.getTarget() + ", priority: " + this.getPriority() +
            ", static: " + this.isStatic();
    }
}
