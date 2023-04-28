package net.titanium.plugins;

import java.util.ArrayList;
import java.util.List;
import net.titanium.$;

/**
 * <h2>PluginLoader</h2>
 */
public class PluginLoader {
    public static final List<TitaniumPlugin> PLUGINS = new ArrayList<>();
    private static final ClassLoader LOADER = PluginLoader.class.getClassLoader();
    @SuppressWarnings("unchecked")
    public static <T extends TitaniumPlugin> void loadPackage(String packageName) {
        try {
            Class<T> clazz = (Class<T>) LOADER.loadClass(packageName);
            T instance = clazz.getDeclaredConstructor().newInstance();
            PLUGINS.add(instance);
        } catch (Exception e) {
            $.LOG.error("Failed to load plugin package: " + packageName, e);
        }
    }

    public static void initPlugins() {
        for(TitaniumPlugin plugin : PLUGINS) {
            plugin.onInit();
        }
    }
}
