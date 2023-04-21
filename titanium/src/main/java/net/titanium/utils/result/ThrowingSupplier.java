package net.titanium.utils.result;

@FunctionalInterface
public interface ThrowingSupplier<S> {
    S get() throws Exception;
}