package net.titanium.plugins;

public abstract class TitaniumPlugin {
    public String name;
    public String[] authors;
    public abstract void onInit();
    public abstract String getPackage();
    public abstract PluginConnections getConnections();
}
