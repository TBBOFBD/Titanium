package net.titanium;

import org.slf4j.Logger;

import net.titanium.composer.*;
import net.titanium.corelog.*;
import net.titanium.plugins.*;
import net.titanium.events.*;

public final class $ extends TitaniumPlugin {
    public static final String NAME = "Titanium";
    public static final String PACKAGE = NAME.toLowerCase();
    public static final Logger LOG = CoreLogger.getLogger(NAME);
    static {
        LOG.trace("Name: '" + NAME + "'");
    }
    public static final IEventBus EVENT_BUS = Composer.getDefault(PACKAGE);
    public static void init() {
        LOG.trace("Loading Packages...");
        PluginLoader.loadPackage($.class.getName());
        LOG.trace("Done Loading Packages");
        LOG.trace("Initializing Packages");
        PluginLoader.initPlugins();
        LOG.trace("Done Initializing Packages");
        EVENT_BUS.post(InitEvent.get());
    }
    @Override
    public void onInit() {
        EVENT_BUS.subscribe(this);
        LOG.trace("Subscribed $.java");
    }
    @Override
    public String getPackage() {
        return PACKAGE;
    }
    @Override
    public PluginConnections getConnections() {
        PluginConnections pc = new PluginConnections();
        pc.addConnection(
            new PluginConnections.GithubRepo("TBBOFBD", NAME)
        );
        return pc;
    }
}