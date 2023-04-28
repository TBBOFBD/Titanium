package net.titanium.composer;

public interface ICancellable {
    void setCancelled(boolean cancelled);
    default void cancel() {
        setCancelled(true);
    }
    boolean isCancelled();
}
