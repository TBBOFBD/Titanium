package net.titanium.composer.core;

public class NoLambdaFactoryRegisteredException extends RuntimeException {
    public NoLambdaFactoryRegisteredException(Class<?> klass) {
        super("No registered lambda listener for '" + klass.getName() + "'.");
    }
}