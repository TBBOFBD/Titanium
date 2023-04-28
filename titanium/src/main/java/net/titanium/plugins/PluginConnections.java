package net.titanium.plugins;

import java.util.ArrayList;
import java.util.List;

@SuppressWarnings("unused")
public class PluginConnections {
    private final List<IConnection> connections;
    public PluginConnections() {
        this.connections = new ArrayList<>();
    }
    public <C extends IConnection> void addConnection(C connection) {
        this.connections.add(connection);
    }
    public IConnection[] getConnections() {
        IConnection[] arr = new IConnection[this.connections.size()];
        return this.connections.toArray(arr);
    }
    public interface IConnection {
        String asString();
        String url();
    }
    public static final class Website implements IConnection {
        private final String name;
        private final String url;
        public Website(String name, String url) {
            this.name = name;
            this.url = url;
        }
        @Override
        public String asString() {
            return this.name;
        }

        @Override
        public String url() {
            return this.url;
        }
    }
    public static final class GithubRepo implements IConnection {
        private final String owner;
        private final String repo;
        public GithubRepo(String owner, String repo) {
            this.owner = owner;
            this.repo = repo;
        }
        @Override
        public String asString() {
            return "github:" + this.owner + "/" + this.repo;
        }
        @Override
        public String url() {
            return "https://www.github.com/" + this.owner + "/" + this.repo;
        }
    }
}
