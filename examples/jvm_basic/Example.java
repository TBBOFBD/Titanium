import net.titanium.composer.*;
import net.titanium.events.*;
import net.titanium.*;

public class Example {
    public static void main(String[] args) { new Main(args); }
    Main(String[] args) {
        Composer.subscribe($.EVENT_BUS,this);
        $.init();
    }

    @EventHandler
    private static void onInit(InitEvent event) {
        $.LOG.info("Info");
    }
}