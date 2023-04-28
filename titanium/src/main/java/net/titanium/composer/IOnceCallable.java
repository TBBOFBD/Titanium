package net.titanium.composer;

public interface IOnceCallable {
    boolean isFirstCall();
    void setAsCalled();
}
